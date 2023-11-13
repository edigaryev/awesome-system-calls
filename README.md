<!-- This file was auto-generated, please edit it's *.j2 template instead -->

# Awesome System Calls

A categorized list of system calls used by popular Unix-like operating systems.

Since the system calls are the primary way of how the OS processes interact with the outside world, this list can be very useful when evaluating what is possible or not on a given operating system.

There are various system call dumps on the internet and attempts to classify and group system calls in various research papers, but it's very hard to find both in the same place. And even if you do, chances are they will be wildly outdated and will only target a single OS.

The *awesome* word is used deliberately as an alias for "collection of", to help people Googling this project.

## System call statistics

* Darwin (🍏) — 453

* FreeBSD (😈) — 392

* Linux (🐧) — 349

* OpenBSD (🐡) — 221

## Table of contents

* [Processes and threads](#processes-and-threads)

  * [Lifecycle](#lifecycle)

    * [Threads](#threads)

  * [File system relationships](#file-system-relationships)

  * [Credentials](#credentials)

    * [UID](#uid)

    * [GID](#gid)

    * [Process groups](#process-groups)

  * [Signals](#signals)

  * [Virtual memory](#virtual-memory)

  * [Timers](#timers)

  * [Sessions](#sessions)

  * [Metadata](#metadata)

    * [Process](#process)

    * [Thread](#thread)

    * [Other](#other)

  * [CPU and NUMA](#cpu-and-numa)

  * [Scheduling](#scheduling)

  * [Resource limits and utilization](#resource-limits-and-utilization)

  * [Terminal](#terminal)

  * [Network routing](#network-routing)

  * [Sleeping](#sleeping)

  * [Locking and synchronization](#locking-and-synchronization)

  * [Operations on other processes](#operations-on-other-processes)

    * [Tracing](#tracing)

    * [Virtual memory](#virtual-memory)

    * [Other](#other)

* [File descriptors and handles](#file-descriptors-and-handles)

  * [Open or create a file](#open-or-create-a-file)

  * [Create a pipe](#create-a-pipe)

  * [Open or create something else](#open-or-create-something-else)

  * [Duplicate a file descriptor](#duplicate-a-file-descriptor)

  * [Modify file descriptor metadata](#modify-file-descriptor-metadata)

  * [Provide file descriptor hints](#provide-file-descriptor-hints)

  * [I/O on a file descriptor](#i-o-on-a-file-descriptor)

    * [Read](#read)

    * [Write](#write)

    * [Zero-copy](#zero-copy)

  * [pidfd (Linux)](#pidfd-linux)

  * [Close a file descriptor](#close-a-file-descriptor)

* [File system](#file-system)

  * [Create an object](#create-an-object)

  * [Modify an objects](#modify-an-objects)

  * [Change object permissions](#change-object-permissions)

  * [Retrieve object stats](#retrieve-object-stats)

  * [Remove object](#remove-object)

  * [Change mount points](#change-mount-points)

  * [Change global file system state](#change-global-file-system-state)

  * [Change object extended attributes](#change-object-extended-attributes)

  * [Watch objects](#watch-objects)

  * [Modify path and FD limits](#modify-path-and-fd-limits)

* [Network](#network)

  * [Berkeley sockets](#berkeley-sockets)

    * [Create](#create)

    * [Socket lifecycle](#socket-lifecycle)

    * [Socket metadata](#socket-metadata)

    * [I/O on a socket](#i-o-on-a-socket)

  * [NFS](#nfs)

* [Polling, multiplexing and asynchronous I/O](#polling-multiplexing-and-asynchronous-i-o)

  * [Polling and multiplexing](#polling-and-multiplexing)

  * [POSIX asynchronous I/O](#posix-asynchronous-i-o)

  * [Asynchronous I/O (Linux)](#asynchronous-i-o-linux)

* [Security](#security)

  * [Sandboxing](#sandboxing)

  * [Jails (FreeBSD)](#jails-free-bsd)

  * [Capabilities (Linux)](#capabilities-linux)

  * [Namespaces (Linux)](#namespaces-linux)

  * [Landlock (Linux)](#landlock-linux)

  * [Entropy and random](#entropy-and-random)

  * [IPSec keys](#ip-sec-keys)

* [Interprocess communication](#interprocess-communication)

  * [System V semaphores](#system-v-semaphores)

  * [POSIX message queues](#posix-message-queues)

  * [Shared memory](#shared-memory)

* [System](#system)

  * [General](#general)

  * [Kernel](#kernel)

    * [Kernel module management (Linux)](#kernel-module-management-linux)

    * [kexec (Linux)](#kexec-linux)

    * [Dynamic kernel linker facility (FreeBSD)](#dynamic-kernel-linker-facility-free-bsd)

  * [System log](#system-log)

  * [Swap](#swap)

  * [Clock and time functions](#clock-and-time-functions)

  * [Quotas and accounting](#quotas-and-accounting)

  * [Performance, profiling and and eBPF](#performance-profiling-and-and-e-bpf)

* [Meta system calls](#meta-system-calls)

## System calls

### Processes and threads

#### Lifecycle

| Name | OS | Description |
|------|----|-------------|
| `fork` | 🍏, 😈, 🐡 | create a child process |
| `vfork` | 🍏, 😈, 🐡 | create a child process and block parent |
| `clone` | 🐧 | create a child process or thread |
| `clone3` | 🐧 | create a child process or thread |
| `posix_spawn` | 🍏 | spawn a process |
| `execve` | 🍏, 😈, 🐧, 🐡 | execute a program |
| `execveat` | 🐧 | execute a program (relative to a directory file) |
| `wait4` | 🍏, 😈, 🐧, 🐡 | wait for process to change state |
| `waitid` | 🍏, 🐧, 🐡 | wait for process to change state |
| `exit` | 🍏, 😈, 🐧, 🐡 | terminate the calling process |

##### Threads

| Name | OS | Description |
|------|----|-------------|
| `tkill` | 🐧 | send a signal to a thread in a thread group |
| `tgkill` | 🐧 | send a signal to a thread |
| `thrkill` | 🐡 | send signal to a thread in the same process |
| `exit_group` | 🐧 | exit all threads in a process's thread group |
| `set_tid_address` | 🐧 | set pointer to thread ID |
| `__get_tcb` | 🐡 | get the address of the thread control block of the current thread |
| `__set_tcb` | 🐡 | set the address of the thread control block of the current thread |
| `__tfork` | 🐡 | create a new kernel thread in the current process |
| `__thrsleep` | 🐡 | userspace thread sleep |
| `__thrwakeup` | 🐡 | userspace thread wakeup |

#### File system relationships

| Name | OS | Description |
|------|----|-------------|
| `getcwd` | 🐧 | get current working directory |
| `__getcwd` | 😈, 🐡 | get current working directory |
| `chdir` | 🍏, 😈, 🐧, 🐡 | set current working directory (by path) |
| `fchdir` | 🍏, 😈, 🐧, 🐡 | set current working directory (by file descriptor) |
| `chroot` | 🍏, 😈, 🐧, 🐡 | change root directory |
| `pivot_root` | 🍏, 🐧 | change the root mount |
| `setfsuid` | 🐧 | set user identity used for filesystem checks |
| `setfsgid` | 🐧 | set group identity used for filesystem checks |

#### Credentials

##### UID

| Name | OS | Description |
|------|----|-------------|
| `getuid` | 🍏, 😈, 🐧, 🐡 |  |
| `setuid` | 🍏, 😈, 🐧, 🐡 |  |
| `geteuid` | 🍏, 😈, 🐧, 🐡 |  |
| `seteuid` | 🍏, 😈, 🐡 |  |
| `getresuid` | 😈, 🐧, 🐡 |  |
| `setresuid` | 😈, 🐧, 🐡 |  |
| `setreuid` | 🍏, 😈, 🐧, 🐡 |  |

##### GID

| Name | OS | Description |
|------|----|-------------|
| `getgid` | 🍏, 😈, 🐧, 🐡 |  |
| `setgid` | 🍏, 😈, 🐧, 🐡 |  |
| `getegid` | 🍏, 😈, 🐧, 🐡 |  |
| `setegid` | 🍏, 😈, 🐡 |  |
| `getresgid` | 😈, 🐧, 🐡 |  |
| `setresgid` | 😈, 🐧, 🐡 |  |
| `setregid` | 🍏, 😈, 🐧, 🐡 |  |
| `getgroups` | 🍏, 😈, 🐧, 🐡 |  |
| `setgroups` | 🍏, 😈, 🐧, 🐡 |  |

##### Process groups

| Name | OS | Description |
|------|----|-------------|
| `getpgid` | 🍏, 😈, 🐧, 🐡 |  |
| `setpgid` | 🍏, 😈, 🐧, 🐡 |  |
| `getpgrp` | 🍏, 😈, 🐡 |  |

#### Signals

| Name | OS | Description |
|------|----|-------------|
| `kill` | 🍏, 😈, 🐧, 🐡 |  |
| `sigaction` | 🍏, 😈, 🐡 |  |
| `rt_sigaction` | 🐧 |  |
| `sigaltstack` | 🍏, 😈, 🐧, 🐡 |  |
| `sigpending` | 🍏, 😈, 🐡 |  |
| `rt_sigpending` | 🐧 |  |
| `sigprocmask` | 🍏, 😈, 🐡 |  |
| `rt_sigprocmask` | 🐧 |  |
| `sigreturn` | 🍏, 😈, 🐡 |  |
| `rt_sigreturn` | 🐧 |  |
| `sigsuspend` | 🍏, 😈, 🐡 |  |
| `rt_sigsuspend` | 🐧 |  |
| `rt_sigtimedwait` | 🐧 |  |
| `rt_sigqueueinfo` | 🐧 |  |
| `rt_tgsigqueueinfo` | 🐧 |  |
| `__thrsigdivert` | 🐡 |  |

#### Virtual memory

| Name | OS | Description |
|------|----|-------------|
| `brk` | 🐧 |  |
| `break` | 😈, 🐡 |  |
| `kbind` | 🐡 |  |
| `minherit` | 🍏, 😈, 🐡 |  |
| `mlock` | 🍏, 😈, 🐧, 🐡 |  |
| `mlockall` | 🍏, 😈, 🐧, 🐡 |  |
| `mmap` | 🍏, 😈, 🐧, 🐡 |  |
| `remap_file_pages` | 🐧 |  |
| `mprotect` | 🍏, 😈, 🐧, 🐡 |  |
| `mimmutable` | 🐡 |  |
| `msync` | 🍏, 😈, 🐧, 🐡 |  |
| `munlock` | 🍏, 😈, 🐧, 🐡 |  |
| `munlockall` | 🍏, 😈, 🐧, 🐡 |  |
| `munmap` | 🍏, 😈, 🐧, 🐡 |  |
| `madvise` | 🍏, 😈, 🐧, 🐡 |  |
| `process_madvise` | 🐧 |  |
| `mquery` | 🐡 |  |
| `mincore` | 🍏, 😈, 🐧 |  |
| `mlock2` | 🐧 |  |
| `mmap2` | 🐧 |  |
| `mbind` | 🐧 |  |
| `mremap` | 🐧 |  |
| `pkey_mprotect` | 🐧 |  |
| `pkey_alloc` | 🐧 |  |
| `pkey_free` | 🐧 |  |

#### Timers

| Name | OS | Description |
|------|----|-------------|
| `getitimer` | 🍏, 😈, 🐧, 🐡 |  |
| `setitimer` | 🍏, 😈, 🐧, 🐡 |  |
| `timer_create` | 🐧 |  |
| `timer_delete` | 🐧 |  |
| `timerfd_gettime` | 🐧 |  |
| `timerfd_settime` | 🐧 |  |
| `timer_getoverrun` | 🐧 |  |
| `timer_gettime` | 🐧 |  |
| `timer_settime` | 🐧 |  |

#### Sessions

| Name | OS | Description |
|------|----|-------------|
| `getlogin_r` | 🐡 |  |
| `getsid` | 🍏, 😈, 🐧, 🐡 |  |
| `setsid` | 🍏, 😈, 🐧, 🐡 |  |
| `setlogin` | 🍏, 😈, 🐡 |  |

#### Metadata

##### Process

| Name | OS | Description |
|------|----|-------------|
| `getpid` | 🍏, 😈, 🐧, 🐡 |  |
| `getppid` | 🍏, 😈, 🐧, 🐡 |  |

##### Thread

| Name | OS | Description |
|------|----|-------------|
| `getthrid` | 🐡 |  |
| `gettid` | 🍏, 🐧 |  |
| `getthrname` | 🐡 |  |
| `setthrname` | 🐡 |  |

##### Other

| Name | OS | Description |
|------|----|-------------|
| `issetugid` | 🍏, 😈, 🐡 |  |
| `personality` | 🐧 |  |

#### CPU and NUMA

| Name | OS | Description |
|------|----|-------------|
| `getcpu` | 🐧 |  |
| `get_mempolicy` | 🐧 |  |
| `set_mempolicy` | 🐧 |  |
| `migrate_pages` | 🐧 |  |
| `move_pages` | 🐧 |  |

#### Scheduling

| Name | OS | Description |
|------|----|-------------|
| `getpriority` | 🍏, 😈, 🐧, 🐡 |  |
| `setpriority` | 🍏, 😈, 🐧, 🐡 |  |
| `sched_yield` | 😈, 🐧, 🐡 |  |
| `sched_getparam` | 😈, 🐧 |  |
| `sched_setparam` | 😈, 🐧 |  |
| `sched_get_priority_min` | 😈, 🐧 |  |
| `sched_get_priority_max` | 😈, 🐧 |  |
| `sched_getscheduler` | 😈, 🐧 |  |
| `sched_setscheduler` | 😈, 🐧 |  |
| `sched_rr_get_interval` | 😈, 🐧 |  |
| `sched_getattr` | 🐧 |  |
| `sched_setattr` | 🐧 |  |
| `sched_getaffinity` | 🐧 |  |
| `sched_setaffinity` | 🐧 |  |
| `ioprio_get` | 🐧 |  |
| `ioprio_set` | 🐧 |  |

#### Resource limits and utilization

| Name | OS | Description |
|------|----|-------------|
| `getrlimit` | 🍏, 😈, 🐧, 🐡 |  |
| `setrlimit` | 🍏, 😈, 🐧, 🐡 |  |
| `prlimit64` | 🐧 |  |
| `getrusage` | 🍏, 😈, 🐧, 🐡 |  |
| `times` | 🐧 |  |

#### Terminal

| Name | OS | Description |
|------|----|-------------|
| `vhangup` | 🐧 |  |

#### Network routing

| Name | OS | Description |
|------|----|-------------|
| `getrtable` | 🐡 |  |
| `setrtable` | 🐡 |  |

#### Sleeping

| Name | OS | Description |
|------|----|-------------|
| `nanosleep` | 😈, 🐧, 🐡 |  |
| `clock_nanosleep` | 😈, 🐧 |  |

#### Locking and synchronization

| Name | OS | Description |
|------|----|-------------|
| `futex` | 🐧, 🐡 |  |
| `futex_waitv` | 🐧 |  |
| `membarrier` | 🐧 |  |
| `get_robust_list` | 🐧 |  |
| `set_robust_list` | 🐧 |  |

#### Operations on other processes

##### Tracing

| Name | OS | Description |
|------|----|-------------|
| `ktrace` | 😈, 🐡 |  |
| `ptrace` | 🍏, 😈, 🐧, 🐡 |  |

##### Virtual memory

| Name | OS | Description |
|------|----|-------------|
| `process_vm_readv` | 🐧 |  |
| `process_vm_writev` | 🐧 |  |

##### Other

| Name | OS | Description |
|------|----|-------------|
| `prctl` | 🐧 |  |
| `kcmp` | 🐧 |  |

### File descriptors and handles

#### Open or create a file

| Name | OS | Description |
|------|----|-------------|
| `open` | 🍏, 😈, 🐡 |  |
| `openat` | 🍏, 😈, 🐧, 🐡 |  |
| `openat2` | 🐧 |  |
| `open_by_handle_at` | 🐧 |  |
| `name_to_handle_at` | 🐧 |  |
| `getfh` | 🍏, 😈, 🐡 |  |
| `fhopen` | 🍏, 😈, 🐡 |  |

#### Create a pipe

| Name | OS | Description |
|------|----|-------------|
| `pipe` | 🍏, 🐡 |  |
| `pipe2` | 😈, 🐧, 🐡 |  |

#### Open or create something else

| Name | OS | Description |
|------|----|-------------|
| `eventfd2` | 🐧 |  |
| `memfd_create` | 🐧 |  |
| `memfd_secret` | 🐧 |  |
| `timerfd_create` | 🐧 |  |
| `userfaultfd` | 🐧 |  |
| `signalfd4` | 🐧 |  |
| `pidfd_open` | 🐧 |  |

#### Duplicate a file descriptor

| Name | OS | Description |
|------|----|-------------|
| `dup` | 🍏, 😈, 🐧, 🐡 |  |
| `dup2` | 🍏, 😈, 🐡 |  |
| `dup3` | 🐧, 🐡 |  |

#### Modify file descriptor metadata

| Name | OS | Description |
|------|----|-------------|
| `fcntl` | 🍏, 😈, 🐧, 🐡 |  |
| `getdtablecount` | 🐡 |  |
| `ioctl` | 🍏, 😈, 🐧, 🐡 |  |
| `lseek` | 🍏, 😈, 🐧, 🐡 |  |
| `llseek` | 🐧 |  |

#### Provide file descriptor hints

| Name | OS | Description |
|------|----|-------------|
| `fadvise64` | 🐧 |  |
| `fadvise64_64` | 🐧 |  |
| `readahead` | 🐧 |  |

#### I/O on a file descriptor

##### Read

| Name | OS | Description |
|------|----|-------------|
| `read` | 🍏, 😈, 🐧, 🐡 |  |
| `readv` | 🍏, 😈, 🐧, 🐡 |  |
| `pread` | 🍏, 😈, 🐡 |  |
| `preadv` | 🍏, 😈, 🐧, 🐡 |  |
| `preadv2` | 🐧 |  |

##### Write

| Name | OS | Description |
|------|----|-------------|
| `write` | 🍏, 😈, 🐧, 🐡 |  |
| `writev` | 🍏, 😈, 🐧, 🐡 |  |
| `pwrite` | 🍏, 😈, 🐡 |  |
| `pwritev` | 🍏, 😈, 🐧, 🐡 |  |
| `pwritev2` | 🐧 |  |

##### Zero-copy

| Name | OS | Description |
|------|----|-------------|
| `copy_file_range` | 😈, 🐧 |  |
| `sendfile` | 🍏, 😈, 🐧 |  |
| `splice` | 🐧 |  |
| `tee` | 🐧 |  |
| `fallocate` | 🐧 |  |
| `vmsplice` | 🐧 |  |

#### pidfd (Linux)

| Name | OS | Description |
|------|----|-------------|
| `pidfd_getfd` | 🐧 |  |
| `pidfd_send_signal` | 🐧 |  |

#### Close a file descriptor

| Name | OS | Description |
|------|----|-------------|
| `close` | 🍏, 😈, 🐧, 🐡 | delete a descriptor |
| `closefrom` | 🐡 | delete many descriptors |
| `close_range` | 😈, 🐧 |  |

### File system

#### Create an object

| Name | OS | Description |
|------|----|-------------|
| `mkdir` | 🍏, 😈, 🐡 |  |
| `mkfifo` | 🍏, 😈, 🐡 |  |
| `mknod` | 🍏, 🐡 |  |
| `link` | 🍏, 😈, 🐡 |  |
| `symlink` | 🍏, 😈, 🐡 |  |
| `symlinkat` | 🍏, 😈, 🐧, 🐡 |  |
| `linkat` | 🍏, 😈, 🐧, 🐡 |  |
| `mkdirat` | 🍏, 😈, 🐧, 🐡 |  |
| `mkfifoat` | 🍏, 😈, 🐡 |  |
| `mknodat` | 🍏, 😈, 🐧, 🐡 |  |

#### Modify an objects

| Name | OS | Description |
|------|----|-------------|
| `rename` | 🍏, 😈, 🐡 |  |
| `renameat` | 🍏, 😈, 🐧, 🐡 |  |
| `renameat2` | 🐧 |  |
| `truncate` | 🍏, 😈, 🐧, 🐡 |  |
| `ftruncate` | 🍏, 😈, 🐧, 🐡 |  |

#### Change object permissions

| Name | OS | Description |
|------|----|-------------|
| `access` | 🍏, 😈, 🐡 |  |
| `faccessat` | 🍏, 😈, 🐧, 🐡 |  |
| `faccessat2` | 🐧 |  |
| `chflags` | 🍏, 😈, 🐡 |  |
| `chmod` | 🍏, 😈, 🐡 |  |
| `chown` | 🍏, 😈, 🐡 |  |
| `lchown` | 🍏, 😈, 🐡 |  |
| `umask` | 🍏, 😈, 🐧, 🐡 |  |
| `chflagsat` | 😈, 🐡 |  |
| `fchmodat` | 🍏, 😈, 🐧, 🐡 |  |
| `fchmodat2` | 🐧 |  |
| `fchownat` | 🍏, 😈, 🐧, 🐡 |  |
| `fchflags` | 🍏, 😈, 🐡 |  |
| `fchmod` | 🍏, 😈, 🐧, 🐡 |  |
| `fchown` | 🍏, 😈, 🐧, 🐡 |  |

#### Retrieve object stats

| Name | OS | Description |
|------|----|-------------|
| `lstat` | 🍏, 🐧, 🐡 |  |
| `fhstat` | 😈, 🐡 |  |
| `fhstatfs` | 😈, 🐡 |  |
| `readlink` | 🍏, 😈, 🐡 |  |
| `__realpath` | 🐡 |  |
| `stat` | 🍏, 🐧, 🐡 |  |
| `statx` | 🐧 |  |
| `fstatat` | 🍏, 😈, 🐡 |  |
| `statfs` | 🍏, 😈, 🐧, 🐡 |  |
| `newfstatat` | 🐧 |  |
| `readlinkat` | 🍏, 😈, 🐧, 🐡 |  |
| `utimes` | 🍏, 😈, 🐡 |  |
| `lutimes` | 😈 |  |

#### Remove object

| Name | OS | Description |
|------|----|-------------|
| `unlink` | 🍏, 😈, 🐡 |  |
| `unlinkat` | 🍏, 😈, 🐧, 🐡 |  |
| `rmdir` | 🍏, 😈, 🐡 |  |
| `utimensat` | 😈, 🐧, 🐡 |  |
| `flock` | 🍏, 😈, 🐧, 🐡 |  |
| `fstat` | 🍏, 😈, 🐧, 🐡 |  |
| `futimens` | 😈, 🐡 |  |
| `futimes` | 🍏, 😈, 🐡 |  |
| `getdents` | 🐡 |  |

#### Change mount points

| Name | OS | Description |
|------|----|-------------|
| `mount` | 🍏, 😈, 🐧, 🐡 |  |
| `getfsstat` | 🍏, 😈, 🐡 |  |
| `fstatfs` | 🍏, 😈, 🐧, 🐡 |  |
| `unmount` | 🍏, 😈, 🐡 |  |
| `umount2` | 🐧 |  |

#### Change global file system state

| Name | OS | Description |
|------|----|-------------|
| `sync` | 🍏, 😈, 🐧, 🐡 |  |
| `fsync` | 🍏, 😈, 🐧, 🐡 |  |
| `fdatasync` | 🍏, 😈, 🐧 |  |
| `sync_file_range` | 🐧 |  |
| `sync_file_range2` | 🐧 |  |
| `syncfs` | 🐧 |  |

#### Change object extended attributes

| Name | OS | Description |
|------|----|-------------|
| `fgetxattr` | 🍏, 🐧 |  |
| `flistxattr` | 🍏, 🐧 |  |
| `fremovexattr` | 🍏, 🐧 |  |
| `fsetxattr` | 🍏, 🐧 |  |
| `getxattr` | 🍏, 🐧 |  |
| `lgetxattr` | 🐧 |  |
| `listxattr` | 🍏, 🐧 |  |
| `llistxattr` | 🐧 |  |
| `lremovexattr` | 🐧 |  |
| `lsetxattr` | 🐧 |  |
| `removexattr` | 🍏, 🐧 |  |
| `setxattr` | 🍏, 🐧 |  |

#### Watch objects

| Name | OS | Description |
|------|----|-------------|
| `inotify_add_watch` | 🐧 |  |
| `inotify_init1` | 🐧 |  |
| `inotify_rm_watch` | 🐧 |  |
| `fanotify_init` | 🐧 |  |
| `fanotify_mark` | 🐧 |  |

#### Modify path and FD limits

| Name | OS | Description |
|------|----|-------------|
| `pathconf` | 🍏, 😈, 🐡 |  |
| `fpathconf` | 🍏, 😈, 🐡 |  |

### Network

#### Berkeley sockets

##### Create

| Name | OS | Description |
|------|----|-------------|
| `socket` | 🍏, 😈, 🐧, 🐡 |  |
| `socketpair` | 🍏, 😈, 🐧, 🐡 |  |

##### Socket lifecycle

| Name | OS | Description |
|------|----|-------------|
| `accept` | 🍏, 😈, 🐧, 🐡 |  |
| `accept4` | 😈, 🐧, 🐡 |  |
| `bind` | 🍏, 😈, 🐧, 🐡 |  |
| `connect` | 🍏, 😈, 🐧, 🐡 |  |
| `listen` | 🍏, 😈, 🐧, 🐡 |  |

##### Socket metadata

| Name | OS | Description |
|------|----|-------------|
| `getpeername` | 🍏, 😈, 🐧, 🐡 |  |
| `getsockname` | 🍏, 😈, 🐧, 🐡 |  |
| `getsockopt` | 🍏, 😈, 🐧, 🐡 |  |
| `setsockopt` | 🍏, 😈, 🐧, 🐡 |  |

##### I/O on a socket

| Name | OS | Description |
|------|----|-------------|
| `recvfrom` | 🍏, 😈, 🐧, 🐡 |  |
| `sendto` | 🍏, 😈, 🐧, 🐡 |  |
| `recvmsg` | 🍏, 😈, 🐧, 🐡 |  |
| `sendmsg` | 🍏, 😈, 🐧, 🐡 |  |
| `recvmmsg` | 🐧, 🐡 |  |
| `sendmmsg` | 🐧, 🐡 |  |

#### NFS

| Name | OS | Description |
|------|----|-------------|
| `nfssvc` | 🍏, 😈, 🐡 |  |
| `nfsservctl` | 🐧 |  |

### Polling, multiplexing and asynchronous I/O

#### Polling and multiplexing

| Name | OS | Description |
|------|----|-------------|
| `kevent` | 🍏, 😈, 🐡 |  |
| `kqueue` | 🍏, 😈, 🐡 |  |
| `kqueue1` | 🐡 |  |
| `poll` | 🍏, 😈, 🐡 |  |
| `ppoll` | 😈, 🐧, 🐡 |  |
| `pselect` | 🍏, 😈, 🐡 |  |
| `select` | 🍏, 😈, 🐡 |  |
| `epoll_create1` | 🐧 |  |
| `epoll_ctl` | 🐧 |  |
| `epoll_pwait` | 🐧 |  |
| `epoll_pwait2` | 🐧 |  |
| `pselect6` | 🐧 |  |

#### POSIX asynchronous I/O

| Name | OS | Description |
|------|----|-------------|
| `aio_read` | 🍏, 😈 |  |
| `aio_write` | 🍏, 😈 |  |
| `aio_return` | 🍏, 😈 |  |
| `aio_suspend` | 🍏, 😈 |  |
| `aio_cancel` | 🍏, 😈 |  |
| `aio_error` | 🍏, 😈 |  |
| `aio_waitcomplete` | 😈 |  |
| `aio_fsync` | 🍏, 😈 |  |
| `aio_mlock` | 😈 |  |

#### Asynchronous I/O (Linux)

| Name | OS | Description |
|------|----|-------------|
| `io_cancel` | 🐧 |  |
| `io_destroy` | 🐧 |  |
| `io_getevents` | 🐧 |  |
| `io_pgetevents` | 🐧 |  |
| `io_setup` | 🐧 |  |
| `io_submit` | 🐧 |  |
| `io_uring_enter` | 🐧 |  |
| `io_uring_register` | 🐧 |  |
| `io_uring_setup` | 🐧 |  |

### Security

#### Sandboxing

| Name | OS | Description |
|------|----|-------------|
| `msyscall` | 🐡 | permit syscalls from a region of pages |
| `pinsyscall` | 🐡 | specify the call stub for a specific system call |
| `pledge` | 🐡 | force the current process into a restricted-service operating mode |
| `revoke` | 🍏, 😈, 🐡 | revoke file access |
| `seccomp` | 🐧 | operate on Secure Computing state of the process |
| `unveil` | 🐡 | unveil parts of a restricted filesystem view |

#### Jails (FreeBSD)

| Name | OS | Description |
|------|----|-------------|
| `jail` | 😈 | sets up a jail and locks the current process in it |
| `jail_attach` | 😈 | attaches the current process to an existing jail |
| `jail_get` | 😈 | retrieves jail parameters |
| `jail_set` | 😈 | creates a new jail or modifies an existing one |
| `jail_remove` | 😈 | removes the jail |

#### Capabilities (Linux)

| Name | OS | Description |
|------|----|-------------|
| `capget` | 🐧 |  |
| `capset` | 🐧 |  |

#### Namespaces (Linux)

| Name | OS | Description |
|------|----|-------------|
| `setns` | 🐧 | reassociate thread with a namespace |
| `unshare` | 🐧 | disassociate parts of the process execution context |

#### Landlock (Linux)

| Name | OS | Description |
|------|----|-------------|
| `landlock_create_ruleset` | 🐧 |  |
| `landlock_add_rule` | 🐧 |  |
| `landlock_restrict_self` | 🐧 |  |

#### Entropy and random

| Name | OS | Description |
|------|----|-------------|
| `getentropy` | 🍏, 🐡 |  |
| `getrandom` | 😈, 🐧 |  |

#### IPSec keys

| Name | OS | Description |
|------|----|-------------|
| `add_key` | 🐧 |  |
| `keyctl` | 🐧 |  |
| `request_key` | 🐧 |  |

### Interprocess communication

#### System V semaphores

| Name | OS | Description |
|------|----|-------------|
| `semctl` | 🍏, 🐧 |  |
| `semtimedop` | 🐧 |  |
| `__semctl` | 😈, 🐡 |  |
| `semget` | 🍏, 😈, 🐧, 🐡 |  |
| `semop` | 🍏, 😈, 🐧, 🐡 |  |

#### POSIX message queues

| Name | OS | Description |
|------|----|-------------|
| `msgctl` | 🍏, 😈, 🐧, 🐡 |  |
| `msgget` | 🍏, 😈, 🐧, 🐡 |  |
| `msgrcv` | 🍏, 😈, 🐧, 🐡 |  |
| `msgsnd` | 🍏, 😈, 🐧, 🐡 |  |
| `mq_getsetattr` | 🐧 |  |
| `mq_notify` | 🐧 |  |
| `mq_open` | 🐧 |  |
| `mq_timedreceive` | 🐧 |  |
| `mq_timedsend` | 🐧 |  |
| `mq_unlink` | 🐧 |  |

#### Shared memory

| Name | OS | Description |
|------|----|-------------|
| `shmat` | 🍏, 😈, 🐧, 🐡 |  |
| `shmctl` | 🍏, 😈, 🐧, 🐡 |  |
| `shmdt` | 🍏, 😈, 🐧, 🐡 |  |
| `shmget` | 🍏, 😈, 🐧, 🐡 |  |

### System

#### General

| Name | OS | Description |
|------|----|-------------|
| `reboot` | 🍏, 😈, 🐧, 🐡 |  |
| `shutdown` | 🍏, 😈, 🐧, 🐡 |  |
| `sysctl` | 🍏, 🐡 |  |
| `setdomainname` | 🐧 |  |
| `sethostname` | 🐧 |  |
| `uname` | 🐧 |  |
| `sysinfo` | 🐧 |  |

#### Kernel

##### Kernel module management (Linux)

| Name | OS | Description |
|------|----|-------------|
| `finit_module` | 🐧 |  |
| `init_module` | 🐧 |  |
| `delete_module` | 🐧 |  |

##### kexec (Linux)

| Name | OS | Description |
|------|----|-------------|
| `kexec_file_load` | 🐧 |  |
| `kexec_load` | 🐧 |  |

##### Dynamic kernel linker facility (FreeBSD)

| Name | OS | Description |
|------|----|-------------|
| `kldsym` | 😈 |  |
| `kldload` | 😈 |  |
| `kldunload` | 😈 |  |
| `kldunloadf` | 😈 |  |
| `kldfind` | 😈 |  |
| `kldnext` | 😈 |  |
| `kldstat` | 😈 |  |
| `kldfirstmod` | 😈 |  |

#### System log

| Name | OS | Description |
|------|----|-------------|
| `sendsyslog` | 🐡 |  |
| `utrace` | 😈, 🐡 |  |
| `syslog` | 🐧 |  |

#### Swap

| Name | OS | Description |
|------|----|-------------|
| `swapctl` | 🐡 |  |
| `swapoff` | 😈, 🐧 |  |
| `swapon` | 🍏, 😈, 🐧 |  |

#### Clock and time functions

| Name | OS | Description |
|------|----|-------------|
| `gettimeofday` | 🍏, 😈, 🐧, 🐡 |  |
| `settimeofday` | 🍏, 😈, 🐧, 🐡 |  |
| `clock_gettime` | 😈, 🐧, 🐡 |  |
| `clock_settime` | 😈, 🐧, 🐡 |  |
| `clock_getres` | 😈, 🐧, 🐡 |  |
| `clock_adjtime` | 🐧 |  |
| `adjtimex` | 🐧 |  |
| `adjfreq` | 🐡 |  |
| `adjtime` | 🍏, 😈, 🐡 |  |

#### Quotas and accounting

| Name | OS | Description |
|------|----|-------------|
| `acct` | 🍏, 😈, 🐧, 🐡 |  |
| `quotactl` | 🍏, 😈, 🐧, 🐡 |  |

#### Performance, profiling and and eBPF

| Name | OS | Description |
|------|----|-------------|
| `bpf` | 🐧 |  |
| `perf_event_open` | 🐧 |  |
| `profil` | 😈, 🐡 |  |

### Meta system calls

| Name | OS | Description |
|------|----|-------------|
| `sysarch` | 😈, 🐡 |  |
| `__syscall` | 😈 |  |
| `syscall` | 🍏, 😈, 🐡 |  |
| `arch_specific_syscall` | 🐧 |  |
| `restart_syscall` | 🐧 |  |
| `rseq` | 🐧 |  |
