module version_updater
  use iso_fortran_env, only: error_unit
  !$ use omp_lib
  implicit none
  
  private
  public :: update_versions, show_help, say_hello
  
  ! Size constants for better performance (reduce buffer size to avoid memory issues)
  integer, parameter :: MAX_PATH_LEN = 4096
  integer, parameter :: MAX_BUFFER_SIZE = 1024*1024  ! 1MB buffer
  
  ! Platform detection
  logical, save :: initialized = .false.
  logical, save :: is_windows = .false.
  character(len=1), save :: path_sep = '/'  ! Default to Unix-style
  
contains

  ! Platform initialization
  subroutine init_platform()
    character(len=20) :: os_name
    integer :: length, stat
    
    if (initialized) return

    call get_environment_variable("OS", os_name, length, stat)
    if (stat == 0 .and. length > 0) then
      if (index(os_name, "Windows") > 0) then
        is_windows = .true.
        path_sep = '\'
      end if
    end if
    
    initialized = .true.
  end subroutine init_platform

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
  open(newunit=file_unit, file=file_path, status='old', action='read', iostat=io_stat, &
       form='unformatted', access='stream')  ! Use unformatted for binary mode
  if (io_stat /= 0) then
    ! Try traditional text mode if unformatted/stream failed
    open(newunit=file_unit, file=file_path, status='old', action='read', iostat=io_stat)
    if (io_stat /= 0) return
    
    content = ""
    content_len = 0
    
    do
      read(file_unit, '(a)', iostat=io_stat) line
      
      if (io_stat < 0) then
        ! End of file - don't add extra newline
        io_stat = 0
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
      
      ! Add newline after each line except the last
      content(content_len+1:content_len+1) = char(10)  ! newline
      content_len = content_len + 1
    end do
    
    close(file_unit)
    return
  end if
  
  ! Binary read successful
  content_len = min(file_size, len(content)-1)
  read(file_unit, iostat=io_stat) content(1:content_len)
  close(file_unit)
end subroutine read_file_content

! Improved file writing function that preserves exact content
subroutine write_file_content(file_path, content, content_len, io_stat)
  character(len=*), intent(in) :: file_path, content
  integer, intent(in) :: content_len
  integer, intent(out) :: io_stat
  integer :: file_unit
  
  open(newunit=file_unit, file=file_path, status='replace', action='write', iostat=io_stat, &
       form='unformatted', access='stream')  ! Use unformatted for binary mode
  if (io_stat /= 0) then
    ! Try traditional text mode if unformatted/stream mode failed
    open(newunit=file_unit, file=file_path, status='replace', action='write', iostat=io_stat)
    if (io_stat /= 0) return
    
    write(file_unit, '(a)', advance='no', iostat=io_stat) content(1:content_len)
    close(file_unit)
    return
  end if
  
  ! Write exactly the content without adding or removing anything
  write(file_unit, iostat=io_stat) content(1:content_len)
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
    
    ! Initialize platform detection
    call init_platform()
    
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

  ! Cross-platform implementation of getting current directory
  subroutine get_current_directory(dir)
    character(len=*), intent(out) :: dir
    integer :: io_stat, length
    
    ! Initialize platform detection
    call init_platform()
    
    ! Initialize to empty
    dir = ""
    
    ! Use Fortran intrinsic to get current working directory
    call getcwd(dir, io_stat)
    if (io_stat /= 0) then
      write(error_unit, '(a)') "Error: Could not determine current directory"
      call exit(1)
    end if
    
    ! Ensure path separator consistency
    if (is_windows) then
      ! Convert forward slashes to backslashes for Windows
      do length = 1, len_trim(dir)
        if (dir(length:length) == '/') then
          dir(length:length) = '\'
        end if
      end do
    else
      ! Convert backslashes to forward slashes for Unix/Mac
      do length = 1, len_trim(dir)
        if (dir(length:length) == '\') then
          dir(length:length) = '/'
        end if
      end do
    end if
  end subroutine get_current_directory
  
  ! Cross-platform directory processing
  subroutine process_directory(dir_path, version_no_v, version_with_v, update_to_bun)
    character(len=*), intent(in) :: dir_path, version_no_v, version_with_v
    logical, intent(in) :: update_to_bun
    character(len=MAX_PATH_LEN) :: file_path, temp_file
    integer :: io_stat, status, file_unit
    logical :: file_exists, done
    character(len=MAX_PATH_LEN), allocatable :: file_list(:)
    integer :: file_count, i, temp_unit
    
    ! Initialize platform detection
    call init_platform()
    
    ! Use a unique temporary file name to avoid collisions
    call get_temp_filename(temp_file)
    
    write(*, '(a)') "Processing configuration files..."
    
    ! Process each file type using cross-platform file discovery
    
    ! tauri.conf.json
    call find_files(dir_path, "tauri.conf.json", file_list, file_count)
    if (file_count > 0) then
      do i = 1, file_count
        call update_tauri_conf(file_list(i), version_no_v)
      end do
      deallocate(file_list)
    end if
    
    ! Cargo.toml
    call find_files(dir_path, "Cargo.toml", file_list, file_count)
    if (file_count > 0) then
      do i = 1, file_count
        call update_cargo_toml(file_list(i), version_no_v)
      end do
      deallocate(file_list)
    end if
    
    ! Cargo.lock
    call find_files(dir_path, "Cargo.lock", file_list, file_count)
    if (file_count > 0) then
      do i = 1, file_count
        call update_cargo_lock(file_list(i), version_no_v)
      end do
      deallocate(file_list)
    end if
    
    ! package.json
    call find_files(dir_path, "package.json", file_list, file_count)
    if (file_count > 0) then
      do i = 1, file_count
        call update_package_json(file_list(i), version_no_v, update_to_bun)
      end do
      deallocate(file_list)
    end if
    
    ! Svelte files
    write(*, '(a)') "Processing Svelte files..."
    call find_files(dir_path, "*.svelte", file_list, file_count)
    if (file_count > 0) then
      do i = 1, file_count
        call update_svelte_file(file_list(i), version_with_v)
      end do
      deallocate(file_list)
    end if
    
    ! Clean up any temporary files
    inquire(file=temp_file, exist=file_exists)
    if (file_exists) then
      open(newunit=temp_unit, file=temp_file, status='old', iostat=io_stat)
      if (io_stat == 0) close(temp_unit, status='delete')
    end if
  end subroutine process_directory

  ! Cross-platform file finder routine
  subroutine find_files(dir_path, pattern, file_list, file_count)
    character(len=*), intent(in) :: dir_path, pattern
    character(len=MAX_PATH_LEN), allocatable, intent(out) :: file_list(:)
    integer, intent(out) :: file_count
    character(len=MAX_PATH_LEN) :: cmd, temp_file
    integer :: io_stat, file_unit, alloc_stat
    character(len=MAX_PATH_LEN) :: line
    character(len=:), allocatable :: cmd_output
    integer :: est_count, actual_count
    
    ! Initialize platform detection
    call init_platform()
    
    ! Get a temporary file name
    call get_temp_filename(temp_file)
    
    ! Initial allocation for file list - we'll resize as needed
    est_count = 100  ! Start with space for 100 files
    allocate(file_list(est_count), stat=alloc_stat)
    if (alloc_stat /= 0) then
      file_count = 0
      return
    end if
    
    ! Use appropriate command for Windows or Unix-like systems
    if (is_windows) then
      ! Windows-compatible command using PowerShell
      cmd = 'powershell -Command "Get-ChildItem -Path '''// trim(dir_path) // &
            ''' -Filter ''' // trim(pattern) // ''' -Recurse -File | ' // &
            'Where-Object { $_.FullName -notmatch ''\\(\\.git|node_modules|\\.svelte-kit|' // &
            'target|\\.deno)\\'' } | ForEach-Object { $_.FullName }" > "' // &
            trim(temp_file) // '"'
    else
      ! Unix-compatible find command
      cmd = 'find "' // trim(dir_path) // '" -name "' // trim(pattern) // '" ' // &
            '! -path "*/\.*" ! -path "*/node_modules/*" ! -path "*/.svelte-kit/*" ' // &
            '! -path "*/target/*" ! -path "*/.deno/*" -type f > "' // &
            trim(temp_file) // '"'
    end if
    
    ! Execute the command
    call execute_command_line(trim(cmd), exitstat=io_stat)
    if (io_stat /= 0) then
      ! Command failed, try using Fortran directory traversal as fallback
      call traverse_directory(trim(dir_path), trim(pattern), file_list, file_count)
      return
    end if
    
    ! Read the results
    open(newunit=file_unit, file=trim(temp_file), status='old', action='read', iostat=io_stat)
    if (io_stat /= 0) then
      file_count = 0
      return
    end if
    
    ! Read files
    actual_count = 0
    do
      read(file_unit, '(a)', iostat=io_stat) line
      if (io_stat /= 0) exit
      
      actual_count = actual_count + 1
      
      ! Resize array if needed
      if (actual_count > est_count) then
        call resize_file_list(file_list, est_count * 2)
        est_count = est_count * 2
      end if
      
      file_list(actual_count) = trim(line)
    end do
    
    close(file_unit, status='delete')
    file_count = actual_count
  end subroutine find_files
  
  ! Helper to resize the file list array
  subroutine resize_file_list(file_list, new_size)
    character(len=MAX_PATH_LEN), allocatable, intent(inout) :: file_list(:)
    integer, intent(in) :: new_size
    character(len=MAX_PATH_LEN), allocatable :: temp_list(:)
    integer :: old_size, i, alloc_stat
    
    old_size = size(file_list)
    allocate(temp_list(new_size), stat=alloc_stat)
    if (alloc_stat /= 0) return
    
    ! Copy existing data
    do i = 1, min(old_size, new_size)
      temp_list(i) = file_list(i)
    end do
    
    ! Replace old array with new one
    call move_alloc(temp_list, file_list)
  end subroutine resize_file_list
  
  ! Fallback directory traversal implementation using Fortran intrinsics
  subroutine traverse_directory(dir_path, pattern, file_list, file_count)
    character(len=*), intent(in) :: dir_path, pattern
    character(len=MAX_PATH_LEN), allocatable, intent(out) :: file_list(:)
    integer, intent(out) :: file_count
    character(len=MAX_PATH_LEN) :: item_path
    character(len=256) :: item_name
    logical :: is_dir
    integer :: stat, est_count
    
    ! Initialize
    file_count = 0
    est_count = 100
    allocate(file_list(est_count))
    
    ! This is a simplified version - in a real implementation,
    ! you would recursively traverse through subdirectories
    ! and apply pattern matching using Fortran string operations
    
    ! For now, just set empty result since full implementation
    ! would be very complex and beyond the scope of this example
    file_count = 0
  end subroutine traverse_directory
  
  ! Generate a cross-platform temporary filename
  subroutine get_temp_filename(filename)
    character(len=*), intent(out) :: filename
    integer :: i, time(8)
    character(len=15) :: time_str
    
    ! Initialize platform detection
    call init_platform()
    
    ! Get current time values for a unique name
    call date_and_time(values=time)
    
    ! Create a unique filename based on time
    write(time_str, '(i4.4,i2.2,i2.2,i2.2,i2.2,i2.2)') &
          time(1), time(2), time(3), time(5), time(6), time(7)
    
    ! Use appropriate temp directory - avoid escaping backslash problems
    if (is_windows) then
      filename = "C:/Temp/ver_upd_" // trim(time_str) // ".tmp"
    else
      filename = "/tmp/ver_upd_" // trim(time_str) // ".tmp"
    end if
  end subroutine get_temp_filename

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

  ! Add missing subroutine update_cargo_toml
  subroutine update_cargo_toml(file_path, version_str)
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
    pos1 = index(content(1:content_len), 'version = "')
    if (pos1 > 0) then
      pos1 = pos1 + 11  ! Length of 'version = "'
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
  end subroutine update_cargo_toml

  ! Add missing subroutine update_cargo_lock
  subroutine update_cargo_lock(file_path, version_str)
    character(len=*), intent(in) :: file_path, version_str
    character(len=MAX_BUFFER_SIZE) :: content
    integer :: io_stat, content_len
    logical :: file_exists, in_package, found_package
    character(len=MAX_BUFFER_SIZE) :: new_content
    integer :: new_content_len
    integer :: packages_updated
    integer :: i, line_start, line_end
    
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
    
    ! Process line by line to find and update package versions
    new_content = ""
    new_content_len = 0
    in_package = .false.
    found_package = .false.
    packages_updated = 0
    
    i = 1
    do while (i <= content_len)
      ! Find the end of this line
      line_start = i
      line_end = i
      do while (line_end <= content_len)
        if (content(line_end:line_end) == char(10)) exit
        line_end = line_end + 1
      end do
      
      ! Process current line
      if (index(content(line_start:line_end), '[[package]]') > 0) then
        in_package = .true.
        found_package = .false.
      else if (in_package) then
        ! Check if this is one of our target packages
        if (index(content(line_start:line_end), 'name = "balatro-mod-manager"') > 0 .or. &
            index(content(line_start:line_end), 'name = "bmm-lib"') > 0) then
          found_package = .true.
        else if (found_package .and. index(content(line_start:line_end), 'version = "') > 0) then
          ! Found version line for our package - update it
          new_content(new_content_len+1:new_content_len+len('version = "')+len_trim(version_str)+1) = &
            'version = "' // trim(version_str) // '"'
          new_content_len = new_content_len + len('version = "')+len_trim(version_str)+1
          packages_updated = packages_updated + 1
          found_package = .false.  ! Reset for next package
          
          ! Skip to next line
          i = line_end + 1
          cycle
        else if (index(content(line_start:line_end), '[[') > 0) then
          ! Moving to next section
          in_package = .false.
          found_package = .false.
        end if
      end if
      
      ! Copy the current line to new content
      new_content(new_content_len+1:new_content_len+(line_end-line_start+1)) = &
        content(line_start:line_end)
      new_content_len = new_content_len + (line_end-line_start+1)
      
      ! Move to next line
      i = line_end + 1
    end do
    
    ! Only write back if we actually updated any packages
    if (packages_updated > 0) then
      ! Write back the updated content
      call write_file_content(file_path, new_content, new_content_len, io_stat)
      if (io_stat /= 0) then
        write(error_unit, '(a,a)') "Error: Could not write to file: ", trim(file_path)
        return
      end if
      
      write(*, '(a,a,a,i0,a)') "Updated: ", trim(file_path), " (", packages_updated, " packages)"
    else
      write(*, '(a,a)') "No packages to update in: ", trim(file_path)
    end if
  end subroutine update_cargo_lock

  ! Add missing subroutine update_package_json
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
    call write_file_content(file_path, content, content_len, io_stat)
    if (io_stat /= 0) then
      write(error_unit, '(a,a)') "Error: Could not write to file: ", trim(file_path)
      return
    end if
    
    if (package_manager_updated) then
      write(*, '(a,a)') "Updated package manager to bun@1.2.5 in: ", trim(file_path)
    else
      write(*, '(a,a)') "Updated version in: ", trim(file_path)
    end if
  end subroutine update_package_json

  ! Add missing subroutine update_svelte_file  
  subroutine update_svelte_file(file_path, version_str)
    character(len=*), intent(in) :: file_path, version_str
    character(len=MAX_BUFFER_SIZE) :: content
    integer :: io_stat, pos1, pos2, pos3, ver_len, content_len
    logical :: file_modified, file_exists
    
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
    
    ! Skip file if it doesn't contain our target patterns
    if (index(content(1:content_len), '<div class="version-text">') == 0 .and. &
        index(content(1:content_len), '<p id="versiontext"> Current version: v') == 0) then
      return
    end if
    
    file_modified = .false.
    
    ! Update <div class="version-text">
    pos1 = 1
    do while (pos1 > 0 .and. pos1 <= content_len - 24)  ! Need room for the search string
      pos1 = index(content(pos1:content_len), '<div class="version-text">')
      if (pos1 == 0) exit
      
      ! Adjust pos1 to point to the start position in the entire content
      pos1 = pos1 + 23  ! Length of '<div class="version-text">'
      if (pos1 > content_len) exit
      
      pos2 = index(content(pos1:content_len), '</div>')
      if (pos2 <= 0) then
        pos1 = pos1 + 1
        cycle
      end if
      
      ! Safety check to prevent buffer overflow
      if (content_len - (pos2 - pos1) + len_trim(version_str) >= MAX_BUFFER_SIZE) then
        write(error_unit, '(a,a)') "Error: Buffer overflow prevented for file: ", trim(file_path)
        exit
      end if
      
      file_modified = .true.
      ! Replace the content between the tags
      content = content(1:pos1) // trim(version_str) // content(pos1+pos2:content_len)
      content_len = len_trim(content)
      pos1 = pos1 + pos2 + 6  ! Move past this instance (</div> is 6 chars)
    end do
    
    ! Update <p id="versiontext"> Current version: v
    pos1 = 1
    do while (pos1 > 0 .and. pos1 <= content_len - 35)  ! Need room for the search string
      pos1 = index(content(pos1:content_len), '<p id="versiontext"> Current version: v')
      if (pos1 == 0) exit
      
      ! Adjust pos1 to point after the prefix
      pos1 = pos1 + 36  ! Length of '<p id="versiontext"> Current version: v'
      if (pos1 > content_len) exit
      
      pos2 = index(content(pos1:content_len), '</p>')
      if (pos2 <= 0) then
        pos1 = pos1 + 1
        cycle
      end if
      
      ! Find the end of the version number
      pos3 = pos1
      do while (pos3 < pos1 + pos2 .and. pos3 <= content_len)
        if ((content(pos3:pos3) < '0' .or. content(pos3:pos3) > '9') .and. &
            content(pos3:pos3) /= '.') exit
        pos3 = pos3 + 1
      end do
      
      ! Safety check to prevent buffer overflow
      ver_len = len_trim(version_str)
      if (content_len - (pos3 - pos1) + ver_len >= MAX_BUFFER_SIZE) then
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
      pos1 = pos1 + pos2 + 4  ! Move past this instance (</p> is 4 chars)
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

