module version_updater
  use iso_fortran_env, only: error_unit
  !$ use omp_lib
  implicit none
  
  private
  public :: update_versions, show_help, say_hello
  
  ! Size constants for better performance (reduce buffer size to avoid memory issues)
  integer, parameter :: MAX_PATH_LEN = 4096
  integer, parameter :: MAX_BUFFER_SIZE = 1024*1024  ! 1MB buffer
  
contains

    ! Improved file reading function to preserve exactly the original file content
subroutine read_file_content(file_path, content, content_len, io_stat)
  character(len=*), intent(in) :: file_path
  character(len=*), intent(out) :: content
  integer, intent(out) :: content_len, io_stat
  integer :: file_unit, file_size
  logical :: file_exists
  character(len=1024) :: line
  
  ! Check if file exists
  inquire(file=file_path, exist=file_exists)
  if (.not. file_exists) then
    io_stat = -1
    return
  end if
  
  ! Get file size to prevent buffer overflow
  inquire(file=file_path, size=file_size)
  if (file_size <= 0 .or. file_size > len(content)-1) then
    io_stat = -2
    return
  end if
  
  ! Read the file while preserving exact format
  open(newunit=file_unit, file=file_path, status='old', action='read', iostat=io_stat)
  if (io_stat /= 0) return
  
  content = ""
  content_len = 0
  
  do
    read(file_unit, '(a)', iostat=io_stat) line
    
    if (io_stat < 0) then
      ! End of file - don't add extra newline
      exit
    else if (io_stat > 0) then
      ! Other error
      close(file_unit)
      return
    end if
    
    ! Add content with careful bounds checking
    if (content_len + len_trim(line) + 1 >= len(content)) then
      io_stat = -3  ! Buffer too small
      close(file_unit)
      return
    end if
    
    ! Append the line
    content(content_len+1:content_len+len_trim(line)) = line(1:len_trim(line))
    content_len = content_len + len_trim(line)
    
    ! Check if we're at the end of the file
    read(file_unit, '(a)', advance='no', iostat=io_stat) line(1:1)
    if (io_stat < 0) then
      ! End of file
      exit
    else if (io_stat > 0) then
      ! Other error
      close(file_unit)
      return
    end if
    
    ! There's more content, so add a newline and backspace to read position
    content(content_len+1:content_len+1) = char(10)  ! newline
    content_len = content_len + 1
    backspace(file_unit)
  end do
  
  close(file_unit)
  io_stat = 0
end subroutine read_file_content

! Improved file writing function that preserves exact content
subroutine write_file_content(file_path, content, content_len, io_stat)
  character(len=*), intent(in) :: file_path, content
  integer, intent(in) :: content_len
  integer, intent(out) :: io_stat
  integer :: file_unit
  
  open(newunit=file_unit, file=file_path, status='replace', action='write', iostat=io_stat)
  if (io_stat /= 0) return
  
  ! Write exactly the content without adding or removing anything
  write(file_unit, '(a)', advance='no', iostat=io_stat) content(1:content_len)
  close(file_unit)
end subroutine write_file_content

  
  subroutine show_help()
    write(*, '(a)') "Usage: version_updater [options] VERSION [DIRECTORY]"
    write(*, '(a)') ""
    write(*, '(a)') "Updates version strings in various files recursively."
    write(*, '(a)') ""
    write(*, '(a)') "Arguments:"
    write(*, '(a)') "  VERSION      Version to set (e.g., v2.0.3)"
    write(*, '(a)') "  DIRECTORY    Directory to scan (default: current directory)"
    write(*, '(a)') ""
    write(*, '(a)') "Options:"
    write(*, '(a)') "  -h, --help   Show this help message and exit"
    write(*, '(a)') "  --bun        Update package.json to use Bun package manager"
    write(*, '(a)') ""
    write(*, '(a)') "Files updated:"
    write(*, '(a)') "  - tauri.conf.json"
    write(*, '(a)') "  - Cargo.toml"
    write(*, '(a)') "  - Cargo.lock (balatro-mod-manager package)"
    write(*, '(a)') "  - package.json"
    write(*, '(a)') "  - All .svelte files containing specific version elements"
    write(*, '(a)') ""
    write(*, '(a)') "Notes:"
    write(*, '(a)') "  - The 'v' prefix is automatically removed for certain files"
    write(*, '(a)') "  - Directories like .git, node_modules, etc. are automatically excluded"
    write(*, '(a)') "  - Use --bun to update package.json to use Bun as the package manager"
  end subroutine show_help
  
  subroutine update_versions(version_arg, directory_arg, use_bun)
    character(len=*), intent(in) :: version_arg
    character(len=*), intent(in), optional :: directory_arg
    logical, intent(in), optional :: use_bun
    
    character(len=100) :: version
    character(len=MAX_PATH_LEN) :: directory
    logical :: has_v_prefix, update_to_bun
    
    ! Check if version starts with 'v' and remove it for certain files
    has_v_prefix = .false.
    if (version_arg(1:1) == 'v') then
      has_v_prefix = .true.
      version = version_arg(2:)
    else
      version = version_arg
    end if
    
    ! Set update_to_bun flag
    update_to_bun = .false.
    if (present(use_bun)) then
      update_to_bun = use_bun
    end if
    
    ! Get current directory if not provided
    if (present(directory_arg)) then
      directory = directory_arg
    else
      call get_current_directory(directory)
    end if
    
    write(*, '(a)') "Starting version update process..."
    write(*, '(a,a)') "Target directory: ", trim(directory)
    write(*, '(a,a)') "Version: ", trim(version_arg)
    if (update_to_bun) then
      write(*, '(a)') "Will update package.json to use Bun package manager"
    end if
    
    !$ write(*, '(a,i0)') "Using OpenMP with threads: ", omp_get_max_threads()
    
    ! Process the directory and update files
    call process_directory(trim(directory), trim(version), trim(version_arg), update_to_bun)
    
    write(*, '(a)') "Version update completed successfully!"
  end subroutine update_versions

  ! Safer implementation of getting current directory
  subroutine get_current_directory(dir)
    character(len=*), intent(out) :: dir
    integer :: io_stat
    character(len=MAX_PATH_LEN) :: temp_dir
    
    ! Initialize to empty
    dir = ""
    
    ! Try using pwd command (safer approach)
    call execute_command_line('pwd > .tmp_dir', exitstat=io_stat)
    if (io_stat /= 0) then
      write(error_unit, '(a)') "Error: Could not determine current directory"
      call exit(1)
    end if
    
    open(unit=10, file='.tmp_dir', status='old', action='read', iostat=io_stat)
    if (io_stat /= 0) then
      write(error_unit, '(a)') "Error: Could not open temporary directory file"
      call exit(1)
    end if
    
    read(10, '(a)', iostat=io_stat) temp_dir
    close(10, status='delete')
    
    if (io_stat /= 0) then
      write(error_unit, '(a)') "Error: Could not read current directory"
      call exit(1)
    end if
    
    ! Check for buffer overflow
    if (len_trim(temp_dir) >= len(dir)) then
      write(error_unit, '(a)') "Error: Directory path too long"
      call exit(1)
    end if
    
    dir = trim(temp_dir)
  end subroutine get_current_directory
  
  ! Simplified directory processing to avoid memory issues
  subroutine process_directory(dir_path, version_no_v, version_with_v, update_to_bun)
    character(len=*), intent(in) :: dir_path, version_no_v, version_with_v
    logical, intent(in) :: update_to_bun
    character(len=MAX_PATH_LEN) :: cmd, file_path
    integer :: io_stat, file_unit
    
    ! First, process specific files directly without storing all paths
    write(*, '(a)') "Processing configuration files..."
    
    ! tauri.conf.json
    cmd = 'find "' // trim(dir_path) // '" -name "tauri.conf.json" ' // &
          '! -path "*/\.*" ! -path "*/node_modules/*" ! -path "*/.svelte-kit/*" ' // &
          '! -path "*/target/*" ! -path "*/.deno/*" -type f -print0 | xargs -0 -I{} echo {}'
          
    call execute_command_line(trim(cmd) // ' > .tmp_files', exitstat=io_stat)
    if (io_stat == 0) then
      open(newunit=file_unit, file='.tmp_files', status='old', action='read', iostat=io_stat)
      if (io_stat == 0) then
        do
          read(file_unit, '(a)', iostat=io_stat) file_path
          if (io_stat /= 0) exit
          call update_tauri_conf(trim(file_path), version_no_v)
        end do
        close(file_unit)
      end if
    end if
    
    ! Cargo.toml
    cmd = 'find "' // trim(dir_path) // '" -name "Cargo.toml" ' // &
          '! -path "*/\.*" ! -path "*/node_modules/*" ! -path "*/.svelte-kit/*" ' // &
          '! -path "*/target/*" ! -path "*/.deno/*" -type f -print0 | xargs -0 -I{} echo {}'
          
    call execute_command_line(trim(cmd) // ' > .tmp_files', exitstat=io_stat)
    if (io_stat == 0) then
      open(newunit=file_unit, file='.tmp_files', status='old', action='read', iostat=io_stat)
      if (io_stat == 0) then
        do
          read(file_unit, '(a)', iostat=io_stat) file_path
          if (io_stat /= 0) exit
          call update_cargo_toml(trim(file_path), version_no_v)
        end do
        close(file_unit)
      end if
    end if
    
    ! Cargo.lock
    cmd = 'find "' // trim(dir_path) // '" -name "Cargo.lock" ' // &
          '! -path "*/\.*" ! -path "*/node_modules/*" ! -path "*/.svelte-kit/*" ' // &
          '! -path "*/target/*" ! -path "*/.deno/*" -type f -print0 | xargs -0 -I{} echo {}'
          
    call execute_command_line(trim(cmd) // ' > .tmp_files', exitstat=io_stat)
    if (io_stat == 0) then
      open(newunit=file_unit, file='.tmp_files', status='old', action='read', iostat=io_stat)
      if (io_stat == 0) then
        do
          read(file_unit, '(a)', iostat=io_stat) file_path
          if (io_stat /= 0) exit
          call update_cargo_lock(trim(file_path), version_no_v)
        end do
        close(file_unit)
      end if
    end if
    
    ! package.json
    cmd = 'find "' // trim(dir_path) // '" -name "package.json" ' // &
          '! -path "*/\.*" ! -path "*/node_modules/*" ! -path "*/.svelte-kit/*" ' // &
          '! -path "*/target/*" ! -path "*/.deno/*" -type f -print0 | xargs -0 -I{} echo {}'
          
    call execute_command_line(trim(cmd) // ' > .tmp_files', exitstat=io_stat)
    if (io_stat == 0) then
      open(newunit=file_unit, file='.tmp_files', status='old', action='read', iostat=io_stat)
      if (io_stat == 0) then
        do
          read(file_unit, '(a)', iostat=io_stat) file_path
          if (io_stat /= 0) exit
          call update_package_json(trim(file_path), version_no_v, update_to_bun)
        end do
        close(file_unit)
      end if
    end if
    
    ! Process Svelte files in chunks to avoid memory issues
    write(*, '(a)') "Processing Svelte files..."
    
    ! First check if any Svelte files exist to avoid unnecessary processing
    cmd = 'find "' // trim(dir_path) // '" -name "*.svelte" ' // &
          '! -path "*/\.*" ! -path "*/node_modules/*" ! -path "*/.svelte-kit/*" ' // &
          '! -path "*/target/*" ! -path "*/.deno/*" -type f -print | head -1'
          
    call execute_command_line(trim(cmd) // ' > .tmp_check', exitstat=io_stat)
    if (io_stat == 0) then
      open(newunit=file_unit, file='.tmp_check', status='old', action='read', iostat=io_stat)
      if (io_stat == 0) then
        read(file_unit, '(a)', iostat=io_stat) file_path
        close(file_unit)
        
        if (io_stat == 0 .and. len_trim(file_path) > 0) then
          ! Svelte files exist, process them
          cmd = 'find "' // trim(dir_path) // '" -name "*.svelte" ' // &
                '! -path "*/\.*" ! -path "*/node_modules/*" ! -path "*/.svelte-kit/*" ' // &
                '! -path "*/target/*" ! -path "*/.deno/*" -type f -print0 | xargs -0 -I{} echo {}'
                
          call execute_command_line(trim(cmd) // ' > .tmp_files', exitstat=io_stat)
          if (io_stat == 0) then
            open(newunit=file_unit, file='.tmp_files', status='old', action='read', iostat=io_stat)
            if (io_stat == 0) then
              ! Process each svelte file
              do
                read(file_unit, '(a)', iostat=io_stat) file_path
                if (io_stat /= 0) exit
                call update_svelte_file(trim(file_path), version_with_v)
              end do
              close(file_unit)
            end if
          end if
        end if
      end if
    end if
    
    ! Clean up temporary files
    call execute_command_line('rm -f .tmp_files .tmp_check', exitstat=io_stat)
  end subroutine process_directory

  subroutine update_tauri_conf(file_path, version_str)
  character(len=*), intent(in) :: file_path, version_str
  character(len=MAX_BUFFER_SIZE) :: content
  integer :: io_stat, pos1, pos2, content_len
  logical :: file_exists
  
  ! Check if file exists
  inquire(file=file_path, exist=file_exists)
  if (.not. file_exists) then
    return
  end if
  
  ! Read the file using our improved reader
  call read_file_content(file_path, content, content_len, io_stat)
  if (io_stat /= 0) then
    write(error_unit, '(a,a)') "Error: Could not read file: ", trim(file_path)
    return
  end if
  
  ! Update the version
  pos1 = index(content(1:content_len), '"version": "')
  if (pos1 > 0) then
    pos1 = pos1 + 12  ! Length of '"version": "'
    pos2 = index(content(pos1:content_len), '"')
    
    if (pos2 > 0) then
      ! Check if we have room for the new version
      if (content_len - (pos1+pos2-1) + len_trim(version_str) < MAX_BUFFER_SIZE) then
        ! Build new content
        content = content(1:pos1-1) // trim(version_str) // content(pos1+pos2-1:content_len)
        content_len = len_trim(content)
        
        ! Write back to file using our improved writer
        call write_file_content(file_path, content, content_len, io_stat)
        if (io_stat /= 0) then
          write(error_unit, '(a,a)') "Error: Could not write to file: ", trim(file_path)
          return
        end if
        
        write(*, '(a,a)') "Updated: ", trim(file_path)
      else
        write(error_unit, '(a,a)') "Error: Buffer overflow prevented for file: ", trim(file_path)
      end if
    end if
  end if
end subroutine update_tauri_conf

 
  subroutine update_package_json(file_path, version_str, update_to_bun)
    character(len=*), intent(in) :: file_path, version_str
    logical, intent(in) :: update_to_bun
    character(len=MAX_BUFFER_SIZE) :: content
    integer :: io_stat, pos1, pos2, content_len
    logical :: file_exists, package_manager_updated
    
    ! Check if file exists
    inquire(file=file_path, exist=file_exists)
    if (.not. file_exists) then
      return
    end if
    
    ! Read the file using our improved reader
    call read_file_content(file_path, content, content_len, io_stat)
    if (io_stat /= 0) then
      write(error_unit, '(a,a)') "Error: Could not read file: ", trim(file_path)
      return
    end if
    
    package_manager_updated = .false.
    
    ! Update the version
    pos1 = index(content(1:content_len), '"version": "')
    if (pos1 > 0) then
      pos1 = pos1 + 12  ! Length of '"version": "'
      pos2 = index(content(pos1:content_len), '"')
      
      if (pos2 > 0) then
        ! Check if we have room for the new version
        if (content_len - (pos1+pos2-1) + len_trim(version_str) < MAX_BUFFER_SIZE) then
          ! Build new content
          content = content(1:pos1-1) // trim(version_str) // content(pos1+pos2-1:content_len)
          content_len = len_trim(content)
        else
          write(error_unit, '(a,a)') "Error: Buffer overflow prevented for file: ", trim(file_path)
        end if
      end if
    end if
    
    ! Update the package manager if requested
    if (update_to_bun) then
      pos1 = index(content(1:content_len), '"packageManager": "')
      if (pos1 > 0) then
        pos1 = pos1 + 18  ! Length of '"packageManager": "'
        pos2 = index(content(pos1:content_len), '"')
        
        if (pos2 > 0) then
          ! Check if we have room for the new package manager
          if (content_len - (pos1+pos2-1) + 10 < MAX_BUFFER_SIZE) then  ! 10 is length of "bun@1.2.5"
            ! Replace with Bun
            content = content(1:pos1-1) // "bun@1.2.5" // content(pos1+pos2-1:content_len)
            content_len = len_trim(content)
            package_manager_updated = .true.
          else
            write(error_unit, '(a,a)') "Error: Buffer overflow prevented for file: ", trim(file_path)
          end if
        end if
      else
        ! packageManager field doesn't exist, add it before the closing brace
        pos1 = content_len
        ! Find the last closing brace
        do while (pos1 > 1)
          if (content(pos1:pos1) == '}') exit
          pos1 = pos1 - 1
        end do
        
        if (pos1 > 1) then
          ! Check if we have comma before this or need to add one
          pos2 = pos1 - 1
          ! Skip whitespace
          do while (pos2 > 1 .and. (content(pos2:pos2) == ' ' .or. content(pos2:pos2) == char(9) .or. &
                  content(pos2:pos2) == char(10) .or. content(pos2:pos2) == char(13)))
            pos2 = pos2 - 1
          end do
          
          ! Check if we have room to add the package manager
          if (content_len + 30 < MAX_BUFFER_SIZE) then  ! 30 is a safe length for new field
            ! Insert packageManager before the closing brace
            content = content(1:pos2) // "," // char(10) // '  "packageManager": "bun@1.2.5"' // &
                      content(pos1:content_len)
            content_len = len_trim(content)
            package_manager_updated = .true.
          else
            write(error_unit, '(a,a)') "Error: Buffer overflow prevented for file: ", trim(file_path)
          end if
        end if
      end if
    end if
    
    ! Write back to file if anything changed
    if (package_manager_updated) then
      call write_file_content(file_path, content, content_len, io_stat)
      if (io_stat /= 0) then
        write(error_unit, '(a,a)') "Error: Could not write to file: ", trim(file_path)
        return
      end if
      
      write(*, '(a,a)') "Updated package manager to bun@1.2.5 in: ", trim(file_path)
    else
      call write_file_content(file_path, content, content_len, io_stat)
      if (io_stat /= 0) then
        write(error_unit, '(a,a)') "Error: Could not write to file: ", trim(file_path)
        return
      end if
      
      write(*, '(a,a)') "Updated version in: ", trim(file_path)
    end if
  end subroutine update_package_json
  
  subroutine update_cargo_toml(file_path, version_str)
    character(len=*), intent(in) :: file_path, version_str
    character(len=MAX_BUFFER_SIZE) :: content
    integer :: io_stat, file_unit, pos1, pos2, content_len, file_size
    logical :: file_exists
    
    ! Check if file exists
    inquire(file=file_path, exist=file_exists)
    if (.not. file_exists) then
      return
    end if
    
    ! Get file size to prevent buffer overflow
    inquire(file=file_path, size=file_size)
    if (file_size <= 0 .or. file_size > MAX_BUFFER_SIZE-1) then
      write(error_unit, '(a,a,a,i0,a)') "Error: File too large or empty: ", trim(file_path), &
            " (", file_size, " bytes)"
      return
    end if
    
    ! Read the file line by line
    open(newunit=file_unit, file=file_path, status='old', action='read', iostat=io_stat)
    if (io_stat /= 0) then
      write(error_unit, '(a,a)') "Error: Could not open file: ", trim(file_path)
      return
    end if
    
    content = ""
    content_len = 0
    do
      if (content_len >= MAX_BUFFER_SIZE - 1024) exit  ! Leave room for safety
      
      read(file_unit, '(a)', iostat=io_stat) content(content_len+1:content_len+1000)
      if (io_stat /= 0) exit
      
      content_len = content_len + len_trim(content(content_len+1:content_len+1000))
      content(content_len+1:content_len+1) = char(10)  ! newline
      content_len = content_len + 1
    end do
    
    close(file_unit)
    
    ! Update the version
    pos1 = index(content(1:content_len), 'version = "')
    if (pos1 > 0) then
      pos1 = pos1 + 11  ! Length of 'version = "'
      pos2 = index(content(pos1:content_len), '"')
      
      if (pos2 > 0) then
        ! Check if we have room for the new version
        if (content_len - (pos1+pos2-1) + len_trim(version_str) < MAX_BUFFER_SIZE) then
          ! Build new content
          content = content(1:pos1-1) // trim(version_str) // content(pos1+pos2-1:content_len)
          
          ! Write back to file
          open(newunit=file_unit, file=file_path, status='replace', action='write', iostat=io_stat)
          if (io_stat /= 0) then
            write(error_unit, '(a,a)') "Error: Could not write to file: ", trim(file_path)
            return
          end if
          
          write(file_unit, '(a)', advance='no') content(1:content_len)
          close(file_unit)
          
          write(*, '(a,a)') "Updated: ", trim(file_path)
        else
          write(error_unit, '(a,a)') "Error: Buffer overflow prevented for file: ", trim(file_path)
        end if
      end if
    end if
  end subroutine update_cargo_toml

subroutine update_cargo_lock(file_path, version_str)
  character(len=*), intent(in) :: file_path, version_str
  character(len=MAX_BUFFER_SIZE) :: content
  integer :: io_stat, file_unit, content_len
  logical :: file_exists, in_package, found_package
  character(len=1024) :: line
  character(len=MAX_BUFFER_SIZE) :: new_content
  integer :: new_content_len
  integer :: packages_updated
  
  ! Check if file exists
  inquire(file=file_path, exist=file_exists)
  if (.not. file_exists) then
    return
  end if
  
  ! First check if any of our target packages exists in the file
  call execute_command_line('grep -q -E "name = \"(balatro-mod-manager|bmm-lib)\"" "' // trim(file_path) // '"', exitstat=io_stat)
  if (io_stat /= 0) then
    ! No target packages found
    return
  end if
  
  ! Process the file manually instead of using sed, which has platform differences
  open(newunit=file_unit, file=file_path, status='old', action='read', iostat=io_stat)
  if (io_stat /= 0) then
    write(error_unit, '(a,a)') "Error: Could not open file: ", trim(file_path)
    return
  end if
  
  ! Read and process line by line
  new_content = ""
  new_content_len = 0
  in_package = .false.
  found_package = .false.
  packages_updated = 0
  
  do
    read(file_unit, '(a)', iostat=io_stat) line
    if (io_stat /= 0) exit
    
    ! Track if we're in a package section
    if (index(line, '[[package]]') > 0) then
      in_package = .true.
      found_package = .false.
    else if (in_package) then
      ! Check if this is one of our target packages
      if (index(line, 'name = "balatro-mod-manager"') > 0 .or. &
          index(line, 'name = "bmm-lib"') > 0) then
        found_package = .true.
      else if (found_package .and. index(line, 'version = "') > 0) then
        ! Found the version line, replace it
        line = 'version = "' // trim(version_str) // '"'
        found_package = .false.  ! Reset for next package
        packages_updated = packages_updated + 1
      else if (index(line, '[[') > 0) then
        ! Moving to next section
        in_package = .false.
        found_package = .false.
      end if
    end if
    
    ! Append line to new content with newline
    if (new_content_len + len_trim(line) + 1 >= MAX_BUFFER_SIZE) then
      close(file_unit)
      write(error_unit, '(a,a)') "Error: Buffer too small for file: ", trim(file_path)
      return
    end if
    
    ! Add the line
    new_content(new_content_len+1:new_content_len+len_trim(line)) = line(1:len_trim(line))
    new_content_len = new_content_len + len_trim(line)
    
    ! Add newline (except for the last line)
    new_content(new_content_len+1:new_content_len+1) = char(10)  ! newline
    new_content_len = new_content_len + 1
  end do
  
  close(file_unit)
  
  ! Only write back if we actually updated any packages
  if (packages_updated > 0) then
    ! Write back the updated content
    open(newunit=file_unit, file=file_path, status='replace', action='write', iostat=io_stat)
    if (io_stat /= 0) then
      write(error_unit, '(a,a)') "Error: Could not write to file: ", trim(file_path)
      return
    end if
    
    write(file_unit, '(a)', advance='no', iostat=io_stat) new_content(1:new_content_len-1)  ! Skip the last newline
    close(file_unit)
    
    write(*, '(a,a,a,i0,a)') "Updated: ", trim(file_path), " (", packages_updated, " packages)"
  else
    write(*, '(a,a)') "No packages to update in: ", trim(file_path)
  end if
end subroutine update_cargo_lock

 
  function system_is_mac() result(is_mac)
    logical :: is_mac
    integer :: io_stat
    character(len=50) :: os_name
    
    call execute_command_line('uname > .tmp_os', exitstat=io_stat)
    if (io_stat /= 0) then
      is_mac = .false.
      return
    end if
    
    open(unit=10, file='.tmp_os', status='old', action='read', iostat=io_stat)
    if (io_stat /= 0) then
      is_mac = .false.
      call execute_command_line('rm -f .tmp_os')
      return
    end if
    
    read(10, '(a)', iostat=io_stat) os_name
    close(10, status='delete')
    
    is_mac = (trim(os_name) == 'Darwin')
  end function system_is_mac

  subroutine update_svelte_file(file_path, version_str)
  character(len=*), intent(in) :: file_path, version_str
  character(len=MAX_BUFFER_SIZE) :: content
  integer :: io_stat, pos1, pos2, pos3, ver_len, content_len, file_size
  logical :: file_modified, file_exists
  character(len=MAX_PATH_LEN) :: grep_cmd
  
  ! Check if file exists
  inquire(file=file_path, exist=file_exists)
  if (.not. file_exists) then
    return
  end if
  
  ! Check if the file contains any of our target patterns before processing
  grep_cmd = 'grep -q -E "(<div class=\"version-text\">|<p id=\"versiontext\"> Current version: v)" "' // &
             trim(file_path) // '"'
  call execute_command_line(trim(grep_cmd), exitstat=io_stat)
  
  if (io_stat /= 0) then
    ! File doesn't contain our patterns, nothing to do
    return
  end if
  
  ! Get file size to prevent buffer overflow
  inquire(file=file_path, size=file_size)
  if (file_size <= 0 .or. file_size > MAX_BUFFER_SIZE-1) then
    write(error_unit, '(a,a,a,i0,a)') "Error: File too large or empty: ", trim(file_path), &
          " (", file_size, " bytes)"
    return
  end if
  
  ! Read the file using our improved reader
  call read_file_content(file_path, content, content_len, io_stat)
  if (io_stat /= 0) then
    write(error_unit, '(a,a)') "Error: Could not read file: ", trim(file_path)
    return
  end if
  
  file_modified = .false.
  
  ! Update <div class="version-text">
  pos1 = 1
  do while (pos1 > 0 .and. pos1 <= content_len - 24)  ! Need room for the search string
    ! FIXED: Use correct closing bracket in search pattern
    pos1 = index(content(pos1:content_len), '<div class="version-text">')
    if (pos1 == 0) exit
    
    ! Adjust pos1 to point after the opening tag
    pos1 = pos1 + len('<div class="version-text">')
    if (pos1 > content_len) exit
    
    pos2 = index(content(pos1:content_len), '</div>')
    if (pos2 <= 0) then
      pos1 = pos1 + 1
      cycle
    end if
    
    ! Safety check to prevent buffer overflow
    if (content_len - (pos1+pos2-1) + len_trim(version_str) >= MAX_BUFFER_SIZE) then
      write(error_unit, '(a,a)') "Error: Buffer overflow prevented for file: ", trim(file_path)
      exit
    end if
    
    file_modified = .true.
    ! Replace the content between the tags
    content = content(1:pos1-1) // trim(version_str) // content(pos1+pos2-1:content_len)
    content_len = len_trim(content)
    pos1 = pos1 + pos2 + 5  ! Move past this instance
  end do
  
  ! Update <p id="versiontext"> Current version: v
  pos1 = 1
  do while (pos1 > 0 .and. pos1 <= content_len - 35)  ! Need room for the search string
    pos1 = index(content(pos1:content_len), '<p id="versiontext"> Current version: v')
    if (pos1 == 0) exit
    
    pos1 = pos1 + 36  ! Length of '<p id="versiontext"> Current version: v'
    if (pos1 > content_len) exit
    
    pos2 = index(content(pos1:content_len), '</p>')
    if (pos2 <= 0) then
      pos1 = pos1 + 1
      cycle
    end if
    
    ! Find the end of the version number
    pos3 = pos1
    do while (pos3 < min(pos1 + pos2, content_len))
      if ((content(pos3:pos3) < '0' .or. content(pos3:pos3) > '9') .and. &
          content(pos3:pos3) /= '.') exit
      pos3 = pos3 + 1
    end do
    
    ! Safety check to prevent buffer overflow
    ver_len = len_trim(version_str)
    if (content_len - pos3 + ver_len >= MAX_BUFFER_SIZE) then
      write(error_unit, '(a,a)') "Error: Buffer overflow prevented for file: ", trim(file_path)
      exit
    end if
    
    file_modified = .true.
    ! Replace just the version number
    if (version_str(1:1) == 'v') then
      ! If version has 'v' prefix, use it from pos1-1
      content = content(1:pos1-1) // version_str(2:ver_len) // content(pos3:content_len)
    else
      content = content(1:pos1-1) // version_str // content(pos3:content_len)
    end if
    
    content_len = len_trim(content)
    pos1 = pos1 + pos2 + 3  ! Move past this instance
  end do
  
  ! Only write file if it was modified
  if (file_modified) then
    ! Write back to file using our improved writer
    call write_file_content(file_path, content, content_len, io_stat)
    if (io_stat /= 0) then
      write(error_unit, '(a,a)') "Error: Could not write to file: ", trim(file_path)
      return
    end if
    
    write(*, '(a,a)') "Updated: ", trim(file_path)
  end if
end subroutine update_svelte_file

  ! Original say_hello subroutine to maintain compatibility
  subroutine say_hello
    print *, "Hello, version_updater!"
  end subroutine say_hello
  
end module version_updater

