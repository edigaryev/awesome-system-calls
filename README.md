<!-- This file was auto-generated, please edit it's *.j2 template instead -->

# Awesome System Calls

A categorized list of system calls used by popular Unix-like operating systems.

Since the system calls are the primary way of how the OS processes interact with the outside world, this list can be very useful when evaluating what is possible or not on a given operating system.

There are various system call dumps on the internet and attempts to classify and group system calls in various research papers, but it's very hard to find both in the same place. And even if you do, chances are they will be wildly outdated and will only target a single OS.

The *awesome* word is used deliberately as an alias for "collection of", to help people Googling this project.

## System call statistics

* OpenBSD (🐡) — 221

* Linux (🐧) — 364

* FreeBSD (😈) — 413

* Darwin (🍏) — 458

## Table of contents

* [Processes and threads](#processes-and-threads)

  * [Lifecycle](#lifecycle)

  * [Lifecycle (thread-specific)](#lifecycle-thread-specific)

  * [File system relationships](#file-system-relationships)

  * [Credentials](#credentials)

    * [UID](#uid)

    * [GID](#gid)

    * [Process groups and sessions](#process-groups-and-sessions)

  * [Signals](#signals)

  * [Virtual memory](#virtual-memory)

    * [Allocation and deallocation](#allocation-and-deallocation)

    * [Locking](#locking)

    * [Protection](#protection)

    * [Hints and synchronization](#hints-and-synchronization)

  * [Timers](#timers)

  * [Metadata](#metadata)

  * [CPU and NUMA](#cpu-and-numa)

  * [Scheduling](#scheduling)

  * [Resource limits and utilization](#resource-limits-and-utilization)

  * [Terminal](#terminal)

  * [Network routing](#network-routing)

  * [Sleeping](#sleeping)

  * [Locking and synchronization](#locking-and-synchronization)

  * [Operations on other processes and threads](#operations-on-other-processes-and-threads)

* [File descriptors and handles](#file-descriptors-and-handles)

  * [Open or create a file](#open-or-create-a-file)

  * [Create a pipe](#create-a-pipe)

  * [Open or create something else](#open-or-create-something-else)

  * [Duplicate a file descriptor](#duplicate-a-file-descriptor)

  * [Modify file descriptor metadata](#modify-file-descriptor-metadata)

  * [Provide file descriptor hints](#provide-file-descriptor-hints)

  * [I/O on a file descriptor](#io-on-a-file-descriptor)

    * [Read](#read)

    * [Write](#write)

    * [Zero-copy](#zero-copy)

  * [Close a file descriptor](#close-a-file-descriptor)

* [File system](#file-system)

  * [Create an object](#create-an-object)

  * [Modify an object](#modify-an-object)

  * [Change object permissions](#change-object-permissions)

  * [Retrieve object stats](#retrieve-object-stats)

  * [Remove object](#remove-object)

  * [Advisory locking](#advisory-locking)

  * [Mount points](#mount-points)

  * [Change global file system state](#change-global-file-system-state)

  * [Change object extended attributes](#change-object-extended-attributes)

  * [Watch objects](#watch-objects)

  * [Retrieve filesystem variables](#retrieve-filesystem-variables)

* [Network](#network)

  * [Berkeley sockets](#berkeley-sockets)

    * [Create](#create)

    * [Socket lifecycle](#socket-lifecycle)

    * [Socket metadata](#socket-metadata)

    * [I/O on a socket](#io-on-a-socket)

  * [NFS](#nfs)

* [Polling, multiplexing and asynchronous I/O](#polling-multiplexing-and-asynchronous-io)

  * [Polling and multiplexing](#polling-and-multiplexing)

  * [POSIX asynchronous I/O](#posix-asynchronous-io)

  * [Asynchronous I/O (Linux)](#asynchronous-io-linux)

  * [io_uring (Linux)](#io_uring-linux)

* [Security](#security)

  * [Sandboxing](#sandboxing)

  * [Jails (FreeBSD)](#jails-freebsd)

  * [Capabilities (Linux)](#capabilities-linux)

  * [Namespaces (Linux)](#namespaces-linux)

  * [Landlock (Linux)](#landlock-linux)

  * [Entropy and random](#entropy-and-random)

  * [Kernel Key Retention Service](#kernel-key-retention-service)

* [Interprocess communication](#interprocess-communication)

  * [System V semaphores](#system-v-semaphores)

  * [POSIX message queues](#posix-message-queues)

  * [Shared memory](#shared-memory)

* [System](#system)

  * [General](#general)

  * [Kernel](#kernel)

    * [Kernel module management (Linux)](#kernel-module-management-linux)

    * [kexec (Linux)](#kexec-linux)

    * [Dynamic kernel linker facility (FreeBSD)](#dynamic-kernel-linker-facility-freebsd)

  * [System log](#system-log)

  * [Swap](#swap)

  * [Clock and time functions](#clock-and-time-functions)

  * [Quotas and accounting](#quotas-and-accounting)

  * [Performance, profiling and and eBPF](#performance-profiling-and-and-ebpf)

* [Meta system calls](#meta-system-calls)

## System calls

### Processes and threads

#### Lifecycle

| Name | OS | Description |
|------|----|-------------|
| `fork` | [🐡](https://man.openbsd.org/fork.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=fork&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fork.2.html) | create a child process |
| `vfork` | [🐡](https://man.openbsd.org/vfork.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=vfork&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/vfork.2.html) | create a child process and block parent |
| `clone` | [🐧](https://man7.org/linux/man-pages/man2/clone.2.html) | create a child process or thread |
| `clone3` | [🐧](https://man7.org/linux/man-pages/man2/clone3.2.html) | create a child process or thread |
| `posix_spawn` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/posix_spawn.2.html) | spawn a process |
| `execve` | [🐡](https://man.openbsd.org/execve.2), [🐧](https://man7.org/linux/man-pages/man2/execve.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=execve&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/execve.2.html) | execute a program |
| `execveat` | [🐧](https://man7.org/linux/man-pages/man2/execveat.2.html) | execute a program (relative to a directory file descriptor) |
| `wait4` | [🐡](https://man.openbsd.org/wait4.2), [🐧](https://man7.org/linux/man-pages/man2/wait4.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=wait4&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/wait4.2.html) | wait for process to change state |
| `waitid` | [🐡](https://man.openbsd.org/waitid.2), [🐧](https://man7.org/linux/man-pages/man2/waitid.2.html), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/waitid.2.html) | wait for process to change state |
| `exit` | [🐡](https://man.openbsd.org/exit.2), [🐧](https://man7.org/linux/man-pages/man2/exit.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=exit&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/exit.2.html) | terminate the calling process |

#### Lifecycle (thread-specific)

| Name | OS | Description |
|------|----|-------------|
| `tkill` | [🐧](https://man7.org/linux/man-pages/man2/tkill.2.html) | send a signal to a thread |
| `tgkill` | [🐧](https://man7.org/linux/man-pages/man2/tgkill.2.html) | send a signal to a thread in a thread group |
| `thrkill` | [🐡](https://man.openbsd.org/thrkill.2) | send a signal to a thread |
| `exit_group` | [🐧](https://man7.org/linux/man-pages/man2/exit_group.2.html) | exit all threads in a process's thread group |
| `set_tid_address` | [🐧](https://man7.org/linux/man-pages/man2/set_tid_address.2.html) | set pointer to a thread ID |
| `__get_tcb` | [🐡](https://man.openbsd.org/__get_tcb.2) | get the address of the thread control block of the current thread |
| `__set_tcb` | [🐡](https://man.openbsd.org/__set_tcb.2) | set the address of the thread control block of the current thread |
| `__tfork` | [🐡](https://man.openbsd.org/__tfork.2) | create a new kernel thread in the current process |
| `__thrsleep` | [🐡](https://man.openbsd.org/__thrsleep.2) | userspace thread sleep |
| `__thrwakeup` | [🐡](https://man.openbsd.org/__thrwakeup.2) | userspace thread wakeup |

#### File system relationships

| Name | OS | Description |
|------|----|-------------|
| `getcwd` | [🐧](https://man7.org/linux/man-pages/man2/getcwd.2.html) | get current working directory |
| `__getcwd` | [🐡](https://man.openbsd.org/__getcwd.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=__getcwd&sektion=2) | get current working directory |
| `chdir` | [🐡](https://man.openbsd.org/chdir.2), [🐧](https://man7.org/linux/man-pages/man2/chdir.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=chdir&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/chdir.2.html) | set current working directory |
| `fchdir` | [🐡](https://man.openbsd.org/fchdir.2), [🐧](https://man7.org/linux/man-pages/man2/fchdir.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fchdir&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fchdir.2.html) | set current working directory (referenced by a file descriptor) |
| `chroot` | [🐡](https://man.openbsd.org/chroot.2), [🐧](https://man7.org/linux/man-pages/man2/chroot.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=chroot&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/chroot.2.html) | change root directory |
| `pivot_root` | [🐧](https://man7.org/linux/man-pages/man2/pivot_root.2.html), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pivot_root.2.html) | change root mount |
| `umask` | [🐡](https://man.openbsd.org/umask.2), [🐧](https://man7.org/linux/man-pages/man2/umask.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=umask&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/umask.2.html) | set file creation mode mask |
| `setfsuid` | [🐧](https://man7.org/linux/man-pages/man2/setfsuid.2.html) | set user identity used for filesystem checks |
| `setfsgid` | [🐧](https://man7.org/linux/man-pages/man2/setfsgid.2.html) | set group identity used for filesystem checks |

#### Credentials

##### UID

| Name | OS | Description |
|------|----|-------------|
| `getuid` | [🐡](https://man.openbsd.org/getuid.2), [🐧](https://man7.org/linux/man-pages/man2/getuid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getuid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getuid.2.html) | get real user ID |
| `setuid` | [🐡](https://man.openbsd.org/setuid.2), [🐧](https://man7.org/linux/man-pages/man2/setuid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setuid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setuid.2.html) | set real user ID |
| `geteuid` | [🐡](https://man.openbsd.org/geteuid.2), [🐧](https://man7.org/linux/man-pages/man2/geteuid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=geteuid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/geteuid.2.html) | get effective user ID |
| `seteuid` | [🐡](https://man.openbsd.org/seteuid.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=seteuid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/seteuid.2.html) | set effective user ID |
| `getresuid` | [🐡](https://man.openbsd.org/getresuid.2), [🐧](https://man7.org/linux/man-pages/man2/getresuid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getresuid&sektion=2) | get real, effective and saved user IDs |
| `setresuid` | [🐡](https://man.openbsd.org/setresuid.2), [🐧](https://man7.org/linux/man-pages/man2/setresuid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setresuid&sektion=2) | set real, effective and saved user IDs |
| `setreuid` | [🐡](https://man.openbsd.org/setreuid.2), [🐧](https://man7.org/linux/man-pages/man2/setreuid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setreuid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setreuid.2.html) | set real and effective user IDs |

##### GID

| Name | OS | Description |
|------|----|-------------|
| `getgid` | [🐡](https://man.openbsd.org/getgid.2), [🐧](https://man7.org/linux/man-pages/man2/getgid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getgid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getgid.2.html) | get real group ID |
| `setgid` | [🐡](https://man.openbsd.org/setgid.2), [🐧](https://man7.org/linux/man-pages/man2/setgid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setgid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setgid.2.html) | set real group ID |
| `getegid` | [🐡](https://man.openbsd.org/getegid.2), [🐧](https://man7.org/linux/man-pages/man2/getegid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getegid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getegid.2.html) | get effective group ID |
| `setegid` | [🐡](https://man.openbsd.org/setegid.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=setegid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setegid.2.html) | set effective group ID |
| `getresgid` | [🐡](https://man.openbsd.org/getresgid.2), [🐧](https://man7.org/linux/man-pages/man2/getresgid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getresgid&sektion=2) | get real, effective and saved group IDs |
| `setresgid` | [🐡](https://man.openbsd.org/setresgid.2), [🐧](https://man7.org/linux/man-pages/man2/setresgid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setresgid&sektion=2) | set real, effective and saved group IDs |
| `setregid` | [🐡](https://man.openbsd.org/setregid.2), [🐧](https://man7.org/linux/man-pages/man2/setregid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setregid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setregid.2.html) | set real and effective group IDs |
| `getgroups` | [🐡](https://man.openbsd.org/getgroups.2), [🐧](https://man7.org/linux/man-pages/man2/getgroups.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getgroups&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getgroups.2.html) | get supplementary group IDs |
| `setgroups` | [🐡](https://man.openbsd.org/setgroups.2), [🐧](https://man7.org/linux/man-pages/man2/setgroups.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setgroups&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setgroups.2.html) | set supplementary group IDs |

##### Process groups and sessions

| Name | OS | Description |
|------|----|-------------|
| `getpgrp` | [🐡](https://man.openbsd.org/getpgrp.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=getpgrp&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getpgrp.2.html) | get process group |
| `getpgid` | [🐡](https://man.openbsd.org/getpgid.2), [🐧](https://man7.org/linux/man-pages/man2/getpgid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getpgid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getpgid.2.html) | get process group (referenced by a process ID) |
| `setpgid` | [🐡](https://man.openbsd.org/setpgid.2), [🐧](https://man7.org/linux/man-pages/man2/setpgid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setpgid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setpgid.2.html) | set process group (referenced by a process ID) |
| `getsid` | [🐡](https://man.openbsd.org/getsid.2), [🐧](https://man7.org/linux/man-pages/man2/getsid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getsid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getsid.2.html) | get process session ID |
| `setsid` | [🐡](https://man.openbsd.org/setsid.2), [🐧](https://man7.org/linux/man-pages/man2/setsid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setsid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setsid.2.html) | set process sesssion ID |
| `getlogin_r` | [🐡](https://man.openbsd.org/getlogin_r.2) | get login name of the user associated with current session |
| `setlogin` | [🐡](https://man.openbsd.org/setlogin.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=setlogin&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setlogin.2.html) | set login name of the user associated with current session |

#### Signals

| Name | OS | Description |
|------|----|-------------|
| `kill` | [🐡](https://man.openbsd.org/kill.2), [🐧](https://man7.org/linux/man-pages/man2/kill.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=kill&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/kill.2.html) | send signal to a process |
| `pidfd_send_signal` | [🐧](https://man7.org/linux/man-pages/man2/pidfd_send_signal.2.html) | send signal to a process (referenced by a file descriptor) |
| `sigaction` | [🐡](https://man.openbsd.org/sigaction.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=sigaction&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sigaction.2.html) | examine and change a signal action |
| `rt_sigaction` | [🐧](https://man7.org/linux/man-pages/man2/rt_sigaction.2.html) | examine and change a signal action |
| `sigaltstack` | [🐡](https://man.openbsd.org/sigaltstack.2), [🐧](https://man7.org/linux/man-pages/man2/sigaltstack.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sigaltstack&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sigaltstack.2.html) | manipulate signal stack context |
| `sigpending` | [🐡](https://man.openbsd.org/sigpending.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=sigpending&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sigpending.2.html) | examine pending signals |
| `rt_sigpending` | [🐧](https://man7.org/linux/man-pages/man2/rt_sigpending.2.html) | examine pending signals |
| `sigprocmask` | [🐡](https://man.openbsd.org/sigprocmask.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=sigprocmask&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sigprocmask.2.html) | examine and change blocked signals |
| `rt_sigprocmask` | [🐧](https://man7.org/linux/man-pages/man2/rt_sigprocmask.2.html) | examine and change blocked signals |
| `sigreturn` | [🐡](https://man.openbsd.org/sigreturn.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=sigreturn&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sigreturn.2.html) | return from the signal handler |
| `rt_sigreturn` | [🐧](https://man7.org/linux/man-pages/man2/rt_sigreturn.2.html) | return from the signal handler |
| `sigsuspend` | [🐡](https://man.openbsd.org/sigsuspend.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=sigsuspend&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sigsuspend.2.html) | wait for signal |
| `rt_sigsuspend` | [🐧](https://man7.org/linux/man-pages/man2/rt_sigsuspend.2.html) | wait for signal |
| `rt_sigtimedwait` | [🐧](https://man7.org/linux/man-pages/man2/rt_sigtimedwait.2.html) | synchronously wait for queued signals |
| `rt_sigqueueinfo` | [🐧](https://man7.org/linux/man-pages/man2/rt_sigqueueinfo.2.html) | queue a signal and data |
| `rt_tgsigqueueinfo` | [🐧](https://man7.org/linux/man-pages/man2/rt_tgsigqueueinfo.2.html) | queue a signal and data |
| `__thrsigdivert` | [🐡](https://man.openbsd.org/__thrsigdivert.2) | synchronously accept a signal |

#### Virtual memory

##### Allocation and deallocation

| Name | OS | Description |
|------|----|-------------|
| `brk` | [🐧](https://man7.org/linux/man-pages/man2/brk.2.html) | change data segment size |
| `mmap` | [🐡](https://man.openbsd.org/mmap.2), [🐧](https://man7.org/linux/man-pages/man2/mmap.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mmap&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mmap.2.html) | map files or devices into memory |
| `mmap2` | [🐧](https://man7.org/linux/man-pages/man2/mmap2.2.html) | map files or devices into memory |
| `mremap` | [🐧](https://man7.org/linux/man-pages/man2/mremap.2.html) | remap a virtual memory address |
| `remap_file_pages` | [🐧](https://man7.org/linux/man-pages/man2/remap_file_pages.2.html) | create a nonlinear file mapping |
| `munmap` | [🐡](https://man.openbsd.org/munmap.2), [🐧](https://man7.org/linux/man-pages/man2/munmap.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=munmap&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/munmap.2.html) | remove a mapping |

##### Locking

| Name | OS | Description |
|------|----|-------------|
| `mlock` | [🐡](https://man.openbsd.org/mlock.2), [🐧](https://man7.org/linux/man-pages/man2/mlock.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mlock&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mlock.2.html) | lock physical pages in memory |
| `mlock2` | [🐧](https://man7.org/linux/man-pages/man2/mlock2.2.html) | lock physical pages in memory |
| `mlockall` | [🐡](https://man.openbsd.org/mlockall.2), [🐧](https://man7.org/linux/man-pages/man2/mlockall.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mlockall&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mlockall.2.html) | lock the address space of a process |
| `mincore` | [🐧](https://man7.org/linux/man-pages/man2/mincore.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mincore&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mincore.2.html) | determine whether pages are resident in memory |
| `munlock` | [🐡](https://man.openbsd.org/munlock.2), [🐧](https://man7.org/linux/man-pages/man2/munlock.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=munlock&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/munlock.2.html) | unlock physical pages in memory |
| `munlockall` | [🐡](https://man.openbsd.org/munlockall.2), [🐧](https://man7.org/linux/man-pages/man2/munlockall.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=munlockall&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/munlockall.2.html) | unlock the address space of a process |

##### Protection

| Name | OS | Description |
|------|----|-------------|
| `mprotect` | [🐡](https://man.openbsd.org/mprotect.2), [🐧](https://man7.org/linux/man-pages/man2/mprotect.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mprotect&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mprotect.2.html) | control the protection of pages |
| `minherit` | [🐡](https://man.openbsd.org/minherit.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=minherit&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/minherit.2.html) | control the inheritance of pages |
| `mimmutable` | [🐡](https://man.openbsd.org/mimmutable.2) | control the immutability of pages |
| `kbind` | [🐡](https://man.openbsd.org/kbind.2) | update protected memory for lazy-binding |
| `pkey_alloc` | [🐧](https://man7.org/linux/man-pages/man2/pkey_alloc.2.html) | allocate a protection key |
| `pkey_mprotect` | [🐧](https://man7.org/linux/man-pages/man2/pkey_mprotect.2.html) | control the protection of pages and their protection keys |
| `pkey_free` | [🐧](https://man7.org/linux/man-pages/man2/pkey_free.2.html) | free a protection key |

##### Hints and synchronization

| Name | OS | Description |
|------|----|-------------|
| `madvise` | [🐡](https://man.openbsd.org/madvise.2), [🐧](https://man7.org/linux/man-pages/man2/madvise.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=madvise&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/madvise.2.html) | give advice about use of memory |
| `process_madvise` | [🐧](https://man7.org/linux/man-pages/man2/process_madvise.2.html) | give advice about use of memory to a process |
| `mquery` | [🐡](https://man.openbsd.org/mquery.2) | provide mapping hints to applications |
| `msync` | [🐡](https://man.openbsd.org/msync.2), [🐧](https://man7.org/linux/man-pages/man2/msync.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=msync&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/msync.2.html) | synchronize a mapped region |

#### Timers

| Name | OS | Description |
|------|----|-------------|
| `getitimer` | [🐡](https://man.openbsd.org/getitimer.2), [🐧](https://man7.org/linux/man-pages/man2/getitimer.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getitimer&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getitimer.2.html) | get value of interval timer |
| `setitimer` | [🐡](https://man.openbsd.org/setitimer.2), [🐧](https://man7.org/linux/man-pages/man2/setitimer.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setitimer&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setitimer.2.html) | set value of interval timer |
| `timer_create` | [🐧](https://man7.org/linux/man-pages/man2/timer_create.2.html) | create a POSIX per-process timer |
| `timer_gettime` | [🐧](https://man7.org/linux/man-pages/man2/timer_gettime.2.html) | fetch state of POSIX per-process timer |
| `timer_settime` | [🐧](https://man7.org/linux/man-pages/man2/timer_settime.2.html) | arm/disarm the POSIX per-process timer |
| `timer_getoverrun` | [🐧](https://man7.org/linux/man-pages/man2/timer_getoverrun.2.html) | get overrun count for a POSIX per-process timer |
| `timer_delete` | [🐧](https://man7.org/linux/man-pages/man2/timer_delete.2.html) | delete a POSIX per-process timer |
| `timerfd_gettime` | [🐧](https://man7.org/linux/man-pages/man2/timerfd_gettime.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=timerfd_gettime&sektion=2) | fetch state of a timer that notifies via file descriptor |
| `timerfd_settime` | [🐧](https://man7.org/linux/man-pages/man2/timerfd_settime.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=timerfd_settime&sektion=2) | arm/disarm a timer that notifies via file descriptor |

#### Metadata

| Name | OS | Description |
|------|----|-------------|
| `getpid` | [🐡](https://man.openbsd.org/getpid.2), [🐧](https://man7.org/linux/man-pages/man2/getpid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getpid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getpid.2.html) | get process ID |
| `getppid` | [🐡](https://man.openbsd.org/getppid.2), [🐧](https://man7.org/linux/man-pages/man2/getppid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getppid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getppid.2.html) | get process ID of the parent |
| `gettid` | [🐧](https://man7.org/linux/man-pages/man2/gettid.2.html), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/gettid.2.html) | get thread ID |
| `getthrid` | [🐡](https://man.openbsd.org/getthrid.2) | get thread ID |
| `getthrname` | [🐡](https://man.openbsd.org/getthrname.2) | get thread name |
| `setthrname` | [🐡](https://man.openbsd.org/setthrname.2) | set thread name |
| `issetugid` | [🐡](https://man.openbsd.org/issetugid.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=issetugid&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/issetugid.2.html) | check if current executable is running `setuid` or `setgid` |
| `personality` | [🐧](https://man7.org/linux/man-pages/man2/personality.2.html) | set the process execution domain |

#### CPU and NUMA

| Name | OS | Description |
|------|----|-------------|
| `getcpu` | [🐧](https://man7.org/linux/man-pages/man2/getcpu.2.html) | determine CPU and NUMA node on which the calling thread is running |
| `get_mempolicy` | [🐧](https://man7.org/linux/man-pages/man2/get_mempolicy.2.html) | retrieve NUMA memory policy for a thread |
| `set_mempolicy` | [🐧](https://man7.org/linux/man-pages/man2/set_mempolicy.2.html) | set default NUMA memory policy for a thread |
| `mbind` | [🐧](https://man7.org/linux/man-pages/man2/mbind.2.html) | set memory policy for a memory range |
| `move_pages` | [🐧](https://man7.org/linux/man-pages/man2/move_pages.2.html) | move individual pages of a process to another node |
| `migrate_pages` | [🐧](https://man7.org/linux/man-pages/man2/migrate_pages.2.html) | move all pages in a process to another set of nodes |

#### Scheduling

| Name | OS | Description |
|------|----|-------------|
| `getpriority` | [🐡](https://man.openbsd.org/getpriority.2), [🐧](https://man7.org/linux/man-pages/man2/getpriority.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getpriority&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getpriority.2.html) | get process scheduling priority |
| `setpriority` | [🐡](https://man.openbsd.org/setpriority.2), [🐧](https://man7.org/linux/man-pages/man2/setpriority.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setpriority&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setpriority.2.html) | set process scheduling priority |
| `sched_yield` | [🐡](https://man.openbsd.org/sched_yield.2), [🐧](https://man7.org/linux/man-pages/man2/sched_yield.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_yield&sektion=2) | yield the processor |
| `sched_getparam` | [🐧](https://man7.org/linux/man-pages/man2/sched_getparam.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_getparam&sektion=2) | get scheduling parameters |
| `sched_setparam` | [🐧](https://man7.org/linux/man-pages/man2/sched_setparam.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_setparam&sektion=2) | set scheduling parameters |
| `sched_getscheduler` | [🐧](https://man7.org/linux/man-pages/man2/sched_getscheduler.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_getscheduler&sektion=2) | get scheduling policy/parameters |
| `sched_setscheduler` | [🐧](https://man7.org/linux/man-pages/man2/sched_setscheduler.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_setscheduler&sektion=2) | set scheduling policy/parameters |
| `sched_getattr` | [🐧](https://man7.org/linux/man-pages/man2/sched_getattr.2.html) | get scheduling policy and attributes |
| `sched_setattr` | [🐧](https://man7.org/linux/man-pages/man2/sched_setattr.2.html) | set scheduling policy and attributes |
| `sched_get_priority_min` | [🐧](https://man7.org/linux/man-pages/man2/sched_get_priority_min.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_get_priority_min&sektion=2) | get static priority range |
| `sched_get_priority_max` | [🐧](https://man7.org/linux/man-pages/man2/sched_get_priority_max.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_get_priority_max&sektion=2) | get static priority range |
| `sched_rr_get_interval` | [🐧](https://man7.org/linux/man-pages/man2/sched_rr_get_interval.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_rr_get_interval&sektion=2) | get the SCHED_RR interval (referenced by a process ID) |
| `sched_getaffinity` | [🐧](https://man7.org/linux/man-pages/man2/sched_getaffinity.2.html) | get thread's CPU affinity mask |
| `sched_setaffinity` | [🐧](https://man7.org/linux/man-pages/man2/sched_setaffinity.2.html) | set thread's CPU affinity mask |
| `ioprio_get` | [🐧](https://man7.org/linux/man-pages/man2/ioprio_get.2.html) | get I/O scheduling class and and priority |
| `ioprio_set` | [🐧](https://man7.org/linux/man-pages/man2/ioprio_set.2.html) | set I/O scheduling class and and priority |

#### Resource limits and utilization

| Name | OS | Description |
|------|----|-------------|
| `getrlimit` | [🐡](https://man.openbsd.org/getrlimit.2), [🐧](https://man7.org/linux/man-pages/man2/getrlimit.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getrlimit&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getrlimit.2.html) | get resource limits |
| `setrlimit` | [🐡](https://man.openbsd.org/setrlimit.2), [🐧](https://man7.org/linux/man-pages/man2/setrlimit.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setrlimit&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setrlimit.2.html) | set resource limits |
| `prlimit64` | [🐧](https://man7.org/linux/man-pages/man2/prlimit64.2.html) | control resource limits (referenced by a process ID) |
| `getrusage` | [🐡](https://man.openbsd.org/getrusage.2), [🐧](https://man7.org/linux/man-pages/man2/getrusage.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getrusage&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getrusage.2.html) | get resource usage |
| `times` | [🐧](https://man7.org/linux/man-pages/man2/times.2.html) | get process times |

#### Terminal

| Name | OS | Description |
|------|----|-------------|
| `vhangup` | [🐧](https://man7.org/linux/man-pages/man2/vhangup.2.html) | virtually hangup the current terminal |

#### Network routing

| Name | OS | Description |
|------|----|-------------|
| `getrtable` | [🐡](https://man.openbsd.org/getrtable.2) | get default routing table of the current process |
| `setrtable` | [🐡](https://man.openbsd.org/setrtable.2) | set default routing table of the current process |

#### Sleeping

| Name | OS | Description |
|------|----|-------------|
| `nanosleep` | [🐡](https://man.openbsd.org/nanosleep.2), [🐧](https://man7.org/linux/man-pages/man2/nanosleep.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=nanosleep&sektion=2) | high-resolution sleep |
| `clock_nanosleep` | [🐧](https://man7.org/linux/man-pages/man2/clock_nanosleep.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=clock_nanosleep&sektion=2) | high-resolution sleep with specifiable clock |

#### Locking and synchronization

| Name | OS | Description |
|------|----|-------------|
| `futex` | [🐡](https://man.openbsd.org/futex.2), [🐧](https://man7.org/linux/man-pages/man2/futex.2.html) | fast userspace locking primitive |
| `futex_waitv` | [🐧](https://man7.org/linux/man-pages/man2/futex_waitv.2.html) | wait on array of futexes |
| `membarrier` | [🐧](https://man7.org/linux/man-pages/man2/membarrier.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=membarrier&sektion=2) | issue memory barriers on a set of threads |
| `get_robust_list` | [🐧](https://man7.org/linux/man-pages/man2/get_robust_list.2.html) | get list of robust futexes |
| `set_robust_list` | [🐧](https://man7.org/linux/man-pages/man2/set_robust_list.2.html) | set list of robust futexes |

#### Operations on other processes and threads

| Name | OS | Description |
|------|----|-------------|
| `ktrace` | [🐡](https://man.openbsd.org/ktrace.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=ktrace&sektion=2) | configure process tracing |
| `ptrace` | [🐡](https://man.openbsd.org/ptrace.2), [🐧](https://man7.org/linux/man-pages/man2/ptrace.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=ptrace&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/ptrace.2.html) | configure process tracing |
| `process_vm_readv` | [🐧](https://man7.org/linux/man-pages/man2/process_vm_readv.2.html) | transfer data between process address spaces |
| `process_vm_writev` | [🐧](https://man7.org/linux/man-pages/man2/process_vm_writev.2.html) | transfer data between process address spaces |
| `prctl` | [🐧](https://man7.org/linux/man-pages/man2/prctl.2.html) | various operations on a process or a thread |
| `kcmp` | [🐧](https://man7.org/linux/man-pages/man2/kcmp.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=kcmp&sektion=2) | compare two processes to determine if they share a kernel resource |

### File descriptors and handles

#### Open or create a file

| Name | OS | Description |
|------|----|-------------|
| `open` | [🐡](https://man.openbsd.org/open.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=open&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/open.2.html) | open or create a file |
| `openat` | [🐡](https://man.openbsd.org/openat.2), [🐧](https://man7.org/linux/man-pages/man2/openat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=openat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/openat.2.html) | open or create a file (relative to a directory file descriptor) |
| `openat2` | [🐧](https://man7.org/linux/man-pages/man2/openat2.2.html) | open or create a file (relative to a directory file descriptor) |
| `getfh` | [🐡](https://man.openbsd.org/getfh.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=getfh&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getfh.2.html) | obtain a handle for a path |
| `name_to_handle_at` | [🐧](https://man7.org/linux/man-pages/man2/name_to_handle_at.2.html) | obtain a handle for a path |
| `fhopen` | [🐡](https://man.openbsd.org/fhopen.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=fhopen&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fhopen.2.html) | open file via a handle |
| `open_by_handle_at` | [🐧](https://man7.org/linux/man-pages/man2/open_by_handle_at.2.html) | open file via a handle |

#### Create a pipe

| Name | OS | Description |
|------|----|-------------|
| `pipe` | [🐡](https://man.openbsd.org/pipe.2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pipe.2.html) | create a pipe |
| `pipe2` | [🐡](https://man.openbsd.org/pipe2.2), [🐧](https://man7.org/linux/man-pages/man2/pipe2.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=pipe2&sektion=2) | create a pipe |

#### Open or create something else

| Name | OS | Description |
|------|----|-------------|
| `eventfd2` | [🐧](https://man7.org/linux/man-pages/man2/eventfd2.2.html) | create a file descriptor for event notification |
| `memfd_create` | [🐧](https://man7.org/linux/man-pages/man2/memfd_create.2.html) | create an anonymous file |
| `memfd_secret` | [🐧](https://man7.org/linux/man-pages/man2/memfd_secret.2.html) | create an anonymous RAM-based file to access secret memory regions |
| `timerfd_create` | [🐧](https://man7.org/linux/man-pages/man2/timerfd_create.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=timerfd_create&sektion=2) | create a timer that notifies via file descriptor |
| `userfaultfd` | [🐧](https://man7.org/linux/man-pages/man2/userfaultfd.2.html) | create a file descriptor for handling page faults in user space |
| `signalfd4` | [🐧](https://man7.org/linux/man-pages/man2/signalfd4.2.html) | create a file descriptor for accepting signals |
| `pidfd_open` | [🐧](https://man7.org/linux/man-pages/man2/pidfd_open.2.html) | obtain a file descriptor that refers to a process |
| `pidfd_getfd` | [🐧](https://man7.org/linux/man-pages/man2/pidfd_getfd.2.html) | obtain a duplicate of another process's file descriptor |

#### Duplicate a file descriptor

| Name | OS | Description |
|------|----|-------------|
| `dup` | [🐡](https://man.openbsd.org/dup.2), [🐧](https://man7.org/linux/man-pages/man2/dup.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=dup&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/dup.2.html) | duplicate an existing file descriptor |
| `dup2` | [🐡](https://man.openbsd.org/dup2.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=dup2&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/dup2.2.html) | duplicate an existing file descriptor |
| `dup3` | [🐡](https://man.openbsd.org/dup3.2), [🐧](https://man7.org/linux/man-pages/man2/dup3.2.html) | duplicate an existing file descriptor |

#### Modify file descriptor metadata

| Name | OS | Description |
|------|----|-------------|
| `fcntl` | [🐡](https://man.openbsd.org/fcntl.2), [🐧](https://man7.org/linux/man-pages/man2/fcntl.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fcntl&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html) | file control |
| `ioctl` | [🐡](https://man.openbsd.org/ioctl.2), [🐧](https://man7.org/linux/man-pages/man2/ioctl.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=ioctl&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/ioctl.2.html) | device control |
| `lseek` | [🐡](https://man.openbsd.org/lseek.2), [🐧](https://man7.org/linux/man-pages/man2/lseek.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=lseek&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/lseek.2.html) | reposition read/write file offset |
| `llseek` | [🐧](https://man7.org/linux/man-pages/man2/llseek.2.html) | reposition read/write file offset |
| `getdtablecount` | [🐡](https://man.openbsd.org/getdtablecount.2) | get descriptor table count |

#### Provide file descriptor hints

| Name | OS | Description |
|------|----|-------------|
| `fadvise64` | [🐧](https://man7.org/linux/man-pages/man2/fadvise64.2.html) | predeclare an access pattern for file data |
| `fadvise64_64` | [🐧](https://man7.org/linux/man-pages/man2/fadvise64_64.2.html) | predeclare an access pattern for file data |
| `readahead` | [🐧](https://man7.org/linux/man-pages/man2/readahead.2.html) | initiate file readahead into page cache |

#### I/O on a file descriptor

##### Read

| Name | OS | Description |
|------|----|-------------|
| `read` | [🐡](https://man.openbsd.org/read.2), [🐧](https://man7.org/linux/man-pages/man2/read.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=read&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/read.2.html) | read from a file descriptor |
| `readv` | [🐡](https://man.openbsd.org/readv.2), [🐧](https://man7.org/linux/man-pages/man2/readv.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=readv&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/readv.2.html) | read from a file descriptor |
| `pread` | [🐡](https://man.openbsd.org/pread.2), [🐧](https://man7.org/linux/man-pages/man2/pread.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=pread&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pread.2.html) | read from a file descriptor at the specified offset |
| `preadv` | [🐡](https://man.openbsd.org/preadv.2), [🐧](https://man7.org/linux/man-pages/man2/preadv.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=preadv&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/preadv.2.html) | read from a file descriptor at the specified offset |
| `preadv2` | [🐧](https://man7.org/linux/man-pages/man2/preadv2.2.html) | read from a file descriptor at the specified offset |

##### Write

| Name | OS | Description |
|------|----|-------------|
| `write` | [🐡](https://man.openbsd.org/write.2), [🐧](https://man7.org/linux/man-pages/man2/write.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=write&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/write.2.html) | write to a file descriptor |
| `writev` | [🐡](https://man.openbsd.org/writev.2), [🐧](https://man7.org/linux/man-pages/man2/writev.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=writev&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/writev.2.html) | write to a file descriptor |
| `pwrite` | [🐡](https://man.openbsd.org/pwrite.2), [🐧](https://man7.org/linux/man-pages/man2/pwrite.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=pwrite&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pwrite.2.html) | write to a file descriptor at the specified offset |
| `pwritev` | [🐡](https://man.openbsd.org/pwritev.2), [🐧](https://man7.org/linux/man-pages/man2/pwritev.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=pwritev&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pwritev.2.html) | write to a file descriptor at the specified offset |
| `pwritev2` | [🐧](https://man7.org/linux/man-pages/man2/pwritev2.2.html) | write to a file descriptor at the specified offset |

##### Zero-copy

| Name | OS | Description |
|------|----|-------------|
| `copy_file_range` | [🐧](https://man7.org/linux/man-pages/man2/copy_file_range.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=copy_file_range&sektion=2) | copy a range of bytes between two file descriptors that refer to files |
| `sendfile` | [🐧](https://man7.org/linux/man-pages/man2/sendfile.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sendfile&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sendfile.2.html) | move data from source (must support `mmap`-like operations) to a destination file descriptor |
| `tee` | [🐧](https://man7.org/linux/man-pages/man2/tee.2.html) | duplicate pipe content |
| `splice` | [🐧](https://man7.org/linux/man-pages/man2/splice.2.html) | move data to/from a pipe |
| `vmsplice` | [🐧](https://man7.org/linux/man-pages/man2/vmsplice.2.html) | splice user pages to/from a pipe |

#### Close a file descriptor

| Name | OS | Description |
|------|----|-------------|
| `close` | [🐡](https://man.openbsd.org/close.2), [🐧](https://man7.org/linux/man-pages/man2/close.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=close&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/close.2.html) | close a file descriptor |
| `closefrom` | [🐡](https://man.openbsd.org/closefrom.2) | close all file descriptors starting from the specified file descriptor |
| `close_range` | [🐧](https://man7.org/linux/man-pages/man2/close_range.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=close_range&sektion=2) | close all file descriptors in a given range |

### File system

#### Create an object

| Name | OS | Description |
|------|----|-------------|
| `mkdir` | [🐡](https://man.openbsd.org/mkdir.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=mkdir&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mkdir.2.html) | create a directory file |
| `mkdirat` | [🐡](https://man.openbsd.org/mkdirat.2), [🐧](https://man7.org/linux/man-pages/man2/mkdirat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mkdirat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mkdirat.2.html) | create a directory file (relative to a directory file descriptor) |
| `mkfifo` | [🐡](https://man.openbsd.org/mkfifo.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=mkfifo&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mkfifo.2.html) | create a FIFO file |
| `mkfifoat` | [🐡](https://man.openbsd.org/mkfifoat.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=mkfifoat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mkfifoat.2.html) | create a FIFO file (relative to a directory file descriptor) |
| `mknod` | [🐡](https://man.openbsd.org/mknod.2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mknod.2.html) | create a special file node |
| `mknodat` | [🐡](https://man.openbsd.org/mknodat.2), [🐧](https://man7.org/linux/man-pages/man2/mknodat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mknodat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mknodat.2.html) | create a special file node (relative to a directory file descriptor) |
| `link` | [🐡](https://man.openbsd.org/link.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=link&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/link.2.html) | create a hard link to file |
| `linkat` | [🐡](https://man.openbsd.org/linkat.2), [🐧](https://man7.org/linux/man-pages/man2/linkat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=linkat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/linkat.2.html) | create a hard link to file (relative to a directory file descriptor) |
| `symlink` | [🐡](https://man.openbsd.org/symlink.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=symlink&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/symlink.2.html) | create a symbolic link to file |
| `symlinkat` | [🐡](https://man.openbsd.org/symlinkat.2), [🐧](https://man7.org/linux/man-pages/man2/symlinkat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=symlinkat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/symlinkat.2.html) | create a symbolic link to file (relative to a directory file descriptor) |

#### Modify an object

| Name | OS | Description |
|------|----|-------------|
| `rename` | [🐡](https://man.openbsd.org/rename.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=rename&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/rename.2.html) | change the name or location of a file |
| `renameat` | [🐡](https://man.openbsd.org/renameat.2), [🐧](https://man7.org/linux/man-pages/man2/renameat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=renameat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/renameat.2.html) | change the name or location of a file (relative to a directory file descriptor) |
| `renameat2` | [🐧](https://man7.org/linux/man-pages/man2/renameat2.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=renameat2&sektion=2) | change the name or location of a file (relative to a directory file descriptor) |
| `renameatx_np` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/renameatx_np.2.html) | change the name or location of a file (relative to a directory file descriptor) |
| `clonefileat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/clonefileat.2.html) | create a copy-on-write clone of files |
| `fclonefileat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fclonefileat.2.html) | create a copy-on-write clone of files (referenced by a file descriptor) |
| `truncate` | [🐡](https://man.openbsd.org/truncate.2), [🐧](https://man7.org/linux/man-pages/man2/truncate.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=truncate&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/truncate.2.html) | truncate or extend a file to a specified length |
| `ftruncate` | [🐡](https://man.openbsd.org/ftruncate.2), [🐧](https://man7.org/linux/man-pages/man2/ftruncate.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=ftruncate&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/ftruncate.2.html) | truncate or extend a file to a specified length (referenced by a file descriptor) |
| `fallocate` | [🐧](https://man7.org/linux/man-pages/man2/fallocate.2.html) | manipulate file space |

#### Change object permissions

| Name | OS | Description |
|------|----|-------------|
| `access` | [🐡](https://man.openbsd.org/access.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=access&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/access.2.html) | check access permissions for a file |
| `faccessat` | [🐡](https://man.openbsd.org/faccessat.2), [🐧](https://man7.org/linux/man-pages/man2/faccessat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=faccessat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/faccessat.2.html) | check access permissions for a file (relative to a directory file descriptor) |
| `faccessat2` | [🐧](https://man7.org/linux/man-pages/man2/faccessat2.2.html) | check access permissions for a file (relative to a directory file descriptor) |
| `chmod` | [🐡](https://man.openbsd.org/chmod.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=chmod&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/chmod.2.html) | change mode of file |
| `fchmod` | [🐡](https://man.openbsd.org/fchmod.2), [🐧](https://man7.org/linux/man-pages/man2/fchmod.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fchmod&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fchmod.2.html) | change mode of file (referenced by a file descriptor) |
| `fchmodat` | [🐡](https://man.openbsd.org/fchmodat.2), [🐧](https://man7.org/linux/man-pages/man2/fchmodat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fchmodat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fchmodat.2.html) | change mode of file (relative to a directory file descriptor) |
| `fchmodat2` | [🐧](https://man7.org/linux/man-pages/man2/fchmodat2.2.html) | change mode of file (relative to a directory file descriptor) |
| `chown` | [🐡](https://man.openbsd.org/chown.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=chown&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/chown.2.html) | change owner and group of a file |
| `lchown` | [🐡](https://man.openbsd.org/lchown.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=lchown&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/lchown.2.html) | change owner and group of a file |
| `fchown` | [🐡](https://man.openbsd.org/fchown.2), [🐧](https://man7.org/linux/man-pages/man2/fchown.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fchown&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fchown.2.html) | change owner and group of a file (referenced by a file descriptor) |
| `fchownat` | [🐡](https://man.openbsd.org/fchownat.2), [🐧](https://man7.org/linux/man-pages/man2/fchownat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fchownat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fchownat.2.html) | change owner and group of a file (relative to a directory file descriptor) |
| `chflags` | [🐡](https://man.openbsd.org/chflags.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=chflags&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/chflags.2.html) | set file flags |
| `fchflags` | [🐡](https://man.openbsd.org/fchflags.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=fchflags&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fchflags.2.html) | set file flags (referenced by a file descriptor) |
| `chflagsat` | [🐡](https://man.openbsd.org/chflagsat.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=chflagsat&sektion=2) | set file flags (relative to a directory file descriptor) |

#### Retrieve object stats

| Name | OS | Description |
|------|----|-------------|
| `readlink` | [🐡](https://man.openbsd.org/readlink.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=readlink&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/readlink.2.html) | read value of a symbolic link |
| `readlinkat` | [🐡](https://man.openbsd.org/readlinkat.2), [🐧](https://man7.org/linux/man-pages/man2/readlinkat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=readlinkat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/readlinkat.2.html) | read value of a symbolic link (relative to a directory file descriptor) |
| `stat` | [🐡](https://man.openbsd.org/stat.2), [🐧](https://man7.org/linux/man-pages/man2/stat.2.html), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/stat.2.html) | get file status |
| `lstat` | [🐡](https://man.openbsd.org/lstat.2), [🐧](https://man7.org/linux/man-pages/man2/lstat.2.html), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/lstat.2.html) | get file status |
| `fstat` | [🐡](https://man.openbsd.org/fstat.2), [🐧](https://man7.org/linux/man-pages/man2/fstat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fstat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fstat.2.html) | get file status (referenced by a file handle) |
| `fstatat` | [🐡](https://man.openbsd.org/fstatat.2), [🐧](https://man7.org/linux/man-pages/man2/fstatat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fstatat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fstatat.2.html) | get file status (relative to a directory file descriptor) |
| `statx` | [🐧](https://man7.org/linux/man-pages/man2/statx.2.html) | get file status (relative to a directory file descriptor) |
| `fhstat` | [🐡](https://man.openbsd.org/fhstat.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=fhstat&sektion=2) | get file status (referenced by a file handle) |
| `statfs` | [🐡](https://man.openbsd.org/statfs.2), [🐧](https://man7.org/linux/man-pages/man2/statfs.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=statfs&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/statfs.2.html) | get file system statistics |
| `fstatfs` | [🐡](https://man.openbsd.org/fstatfs.2), [🐧](https://man7.org/linux/man-pages/man2/fstatfs.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fstatfs&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fstatfs.2.html) | get file system statistics (referenced by a file descriptor) |
| `fhstatfs` | [🐡](https://man.openbsd.org/fhstatfs.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=fhstatfs&sektion=2) | get file system statistics (referenced by a file handle) |
| `newfstatat` | [🐧](https://man7.org/linux/man-pages/man2/newfstatat.2.html) | get file system statistics (relative to a directory file descriptor) |
| `utimes` | [🐡](https://man.openbsd.org/utimes.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=utimes&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/utimes.2.html) | set file access and modification times |
| `lutimes` | [😈](https://www.freebsd.org/cgi/man.cgi?query=lutimes&sektion=2) | set file access and modification times |
| `utimensat` | [🐡](https://man.openbsd.org/utimensat.2), [🐧](https://man7.org/linux/man-pages/man2/utimensat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=utimensat&sektion=2) | set file access and modification times (relative to a directory file descriptor) |
| `futimes` | [🐡](https://man.openbsd.org/futimes.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=futimes&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/futimes.2.html) | set file access and modification times |
| `futimens` | [🐡](https://man.openbsd.org/futimens.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=futimens&sektion=2) | set file access and modification times |
| `getdents` | [🐡](https://man.openbsd.org/getdents.2), [🐧](https://man7.org/linux/man-pages/man2/getdents.2.html) | get directory entries in a filesystem independent format |

#### Remove object

| Name | OS | Description |
|------|----|-------------|
| `unlink` | [🐡](https://man.openbsd.org/unlink.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=unlink&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/unlink.2.html) | remove directory entry |
| `unlinkat` | [🐡](https://man.openbsd.org/unlinkat.2), [🐧](https://man7.org/linux/man-pages/man2/unlinkat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=unlinkat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/unlinkat.2.html) | remove directory entry (relative to a directory file descriptor) |
| `rmdir` | [🐡](https://man.openbsd.org/rmdir.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=rmdir&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/rmdir.2.html) | remove a directory file |

#### Advisory locking

| Name | OS | Description |
|------|----|-------------|
| `flock` | [🐡](https://man.openbsd.org/flock.2), [🐧](https://man7.org/linux/man-pages/man2/flock.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=flock&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/flock.2.html) | apply or remove an advisory lock on an open file |

#### Mount points

| Name | OS | Description |
|------|----|-------------|
| `mount` | [🐡](https://man.openbsd.org/mount.2), [🐧](https://man7.org/linux/man-pages/man2/mount.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mount&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mount.2.html) | mount a filesystem |
| `getfsstat` | [🐡](https://man.openbsd.org/getfsstat.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=getfsstat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getfsstat.2.html) | get list of all mounted file systems |
| `unmount` | [🐡](https://man.openbsd.org/unmount.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=unmount&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/unmount.2.html) | dismount a filesystem |
| `umount2` | [🐧](https://man7.org/linux/man-pages/man2/umount2.2.html) | dismount a filesystem |

#### Change global file system state

| Name | OS | Description |
|------|----|-------------|
| `sync` | [🐡](https://man.openbsd.org/sync.2), [🐧](https://man7.org/linux/man-pages/man2/sync.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sync&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sync.2.html) | synchronize disk in-core state with storage device |
| `fsync` | [🐡](https://man.openbsd.org/fsync.2), [🐧](https://man7.org/linux/man-pages/man2/fsync.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fsync&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fsync.2.html) | synchronize file in-core state with storage device |
| `fdatasync` | [🐧](https://man7.org/linux/man-pages/man2/fdatasync.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fdatasync&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fdatasync.2.html) | synchronize file in-core state with storage device |
| `sync_file_range` | [🐧](https://man7.org/linux/man-pages/man2/sync_file_range.2.html) | synchronize file's segment with disk |
| `sync_file_range2` | [🐧](https://man7.org/linux/man-pages/man2/sync_file_range2.2.html) | synchronize file's segment with disk |
| `syncfs` | [🐧](https://man7.org/linux/man-pages/man2/syncfs.2.html) | commit filesystem caches to disk |

#### Change object extended attributes

| Name | OS | Description |
|------|----|-------------|
| `getxattr` | [🐧](https://man7.org/linux/man-pages/man2/getxattr.2.html), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getxattr.2.html) | retrieve an extended attribute value |
| `lgetxattr` | [🐧](https://man7.org/linux/man-pages/man2/lgetxattr.2.html) | retrieve an extended attribute value |
| `fgetxattr` | [🐧](https://man7.org/linux/man-pages/man2/fgetxattr.2.html), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fgetxattr.2.html) | retrieve an extended attribute value (referenced by a file descriptor) |
| `setxattr` | [🐧](https://man7.org/linux/man-pages/man2/setxattr.2.html), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setxattr.2.html) | set an extended attribute value |
| `lsetxattr` | [🐧](https://man7.org/linux/man-pages/man2/lsetxattr.2.html) | set an extended attribute value |
| `fsetxattr` | [🐧](https://man7.org/linux/man-pages/man2/fsetxattr.2.html), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fsetxattr.2.html) | set an extended attribute value (referenced by a file descriptor) |
| `listxattr` | [🐧](https://man7.org/linux/man-pages/man2/listxattr.2.html), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/listxattr.2.html) | list extended attribute names |
| `llistxattr` | [🐧](https://man7.org/linux/man-pages/man2/llistxattr.2.html) | list extended attribute names |
| `flistxattr` | [🐧](https://man7.org/linux/man-pages/man2/flistxattr.2.html), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/flistxattr.2.html) | list extended attribute names (referenced by a file descriptor) |
| `removexattr` | [🐧](https://man7.org/linux/man-pages/man2/removexattr.2.html), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/removexattr.2.html) | remove an extended attribute |
| `lremovexattr` | [🐧](https://man7.org/linux/man-pages/man2/lremovexattr.2.html) | remove an extended attribute |
| `fremovexattr` | [🐧](https://man7.org/linux/man-pages/man2/fremovexattr.2.html), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fremovexattr.2.html) | remove an extended attribute (referenced by a file descriptor) |

#### Watch objects

| Name | OS | Description |
|------|----|-------------|
| `inotify_init1` | [🐧](https://man7.org/linux/man-pages/man2/inotify_init1.2.html) | initialize an [`inotify(7)`](https://man7.org/linux/man-pages/man7/inotify.7.html) instance |
| `inotify_add_watch` | [🐧](https://man7.org/linux/man-pages/man2/inotify_add_watch.2.html) | add a watch to an initialized `inotify(7)` instance |
| `inotify_rm_watch` | [🐧](https://man7.org/linux/man-pages/man2/inotify_rm_watch.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=inotify_rm_watch&sektion=2) | remove an existing watch from an `inotify(7)` instance |
| `fanotify_init` | [🐧](https://man7.org/linux/man-pages/man2/fanotify_init.2.html) | create and initialize [`fanotify(7)`](https://man7.org/linux/man-pages/man7/fanotify.7.html) group |
| `fanotify_mark` | [🐧](https://man7.org/linux/man-pages/man2/fanotify_mark.2.html) | add, remove, or modify an `fanotify(7)` mark on a filesystem object |

#### Retrieve filesystem variables

| Name | OS | Description |
|------|----|-------------|
| `pathconf` | [🐡](https://man.openbsd.org/pathconf.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=pathconf&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pathconf.2.html) | get configurable pathname variables |
| `fpathconf` | [🐡](https://man.openbsd.org/fpathconf.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=fpathconf&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fpathconf.2.html) | get configurable pathname variables (referenced by a file descriptor) |

### Network

#### Berkeley sockets

##### Create

| Name | OS | Description |
|------|----|-------------|
| `socket` | [🐡](https://man.openbsd.org/socket.2), [🐧](https://man7.org/linux/man-pages/man2/socket.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=socket&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/socket.2.html) | create an endpoint for communication |
| `socketpair` | [🐡](https://man.openbsd.org/socketpair.2), [🐧](https://man7.org/linux/man-pages/man2/socketpair.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=socketpair&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/socketpair.2.html) | create a pair of connected sockets |

##### Socket lifecycle

| Name | OS | Description |
|------|----|-------------|
| `connect` | [🐡](https://man.openbsd.org/connect.2), [🐧](https://man7.org/linux/man-pages/man2/connect.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=connect&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/connect.2.html) | initiate a connection on a socket |
| `bind` | [🐡](https://man.openbsd.org/bind.2), [🐧](https://man7.org/linux/man-pages/man2/bind.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=bind&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/bind.2.html) | bind a name to a socket |
| `listen` | [🐡](https://man.openbsd.org/listen.2), [🐧](https://man7.org/linux/man-pages/man2/listen.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=listen&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/listen.2.html) | listen for connections on a socket |
| `accept` | [🐡](https://man.openbsd.org/accept.2), [🐧](https://man7.org/linux/man-pages/man2/accept.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=accept&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/accept.2.html) | accept a connection on a socket |
| `accept4` | [🐡](https://man.openbsd.org/accept4.2), [🐧](https://man7.org/linux/man-pages/man2/accept4.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=accept4&sektion=2) | accept a connection on a socket |

##### Socket metadata

| Name | OS | Description |
|------|----|-------------|
| `getsockname` | [🐡](https://man.openbsd.org/getsockname.2), [🐧](https://man7.org/linux/man-pages/man2/getsockname.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getsockname&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getsockname.2.html) | get local protocol address associated with a socket |
| `getpeername` | [🐡](https://man.openbsd.org/getpeername.2), [🐧](https://man7.org/linux/man-pages/man2/getpeername.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getpeername&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getpeername.2.html) | get remote protocol address associated with a socket |
| `getsockopt` | [🐡](https://man.openbsd.org/getsockopt.2), [🐧](https://man7.org/linux/man-pages/man2/getsockopt.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getsockopt&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getsockopt.2.html) | get socket options |
| `setsockopt` | [🐡](https://man.openbsd.org/setsockopt.2), [🐧](https://man7.org/linux/man-pages/man2/setsockopt.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setsockopt&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setsockopt.2.html) | set socket options |

##### I/O on a socket

| Name | OS | Description |
|------|----|-------------|
| `recvfrom` | [🐡](https://man.openbsd.org/recvfrom.2), [🐧](https://man7.org/linux/man-pages/man2/recvfrom.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=recvfrom&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/recvfrom.2.html) | receive a message from a socket |
| `recvmsg` | [🐡](https://man.openbsd.org/recvmsg.2), [🐧](https://man7.org/linux/man-pages/man2/recvmsg.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=recvmsg&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/recvmsg.2.html) | receive a message from a socket |
| `recvmmsg` | [🐡](https://man.openbsd.org/recvmmsg.2), [🐧](https://man7.org/linux/man-pages/man2/recvmmsg.2.html) | receive multiple messages from a socket |
| `recvmsg_x` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/recvmsg_x.2.html) | receive multiple datagrams from a socket |
| `sendto` | [🐡](https://man.openbsd.org/sendto.2), [🐧](https://man7.org/linux/man-pages/man2/sendto.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sendto&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sendto.2.html) | send a message on a socket |
| `sendmsg` | [🐡](https://man.openbsd.org/sendmsg.2), [🐧](https://man7.org/linux/man-pages/man2/sendmsg.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sendmsg&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sendmsg.2.html) | send a message on a socket |
| `sendmmsg` | [🐡](https://man.openbsd.org/sendmmsg.2), [🐧](https://man7.org/linux/man-pages/man2/sendmmsg.2.html) | send multiple messages on a socket |
| `sendmsg_x` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sendmsg_x.2.html) | send multiple datagrams on a socket |
| `shutdown` | [🐡](https://man.openbsd.org/shutdown.2), [🐧](https://man7.org/linux/man-pages/man2/shutdown.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=shutdown&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/shutdown.2.html) | disable sends or receives on a socket |

#### NFS

| Name | OS | Description |
|------|----|-------------|
| `nfssvc` | [🐡](https://man.openbsd.org/nfssvc.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=nfssvc&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/nfssvc.2.html) | NFS daemon services |
| `nfsservctl` | [🐧](https://man7.org/linux/man-pages/man2/nfsservctl.2.html) | NFS daemon services |

### Polling, multiplexing and asynchronous I/O

#### Polling and multiplexing

| Name | OS | Description |
|------|----|-------------|
| `select` | [🐡](https://man.openbsd.org/select.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=select&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/select.2.html) | synchronous I/O multiplexing |
| `pselect` | [🐡](https://man.openbsd.org/pselect.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=pselect&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pselect.2.html) | synchronous I/O multiplexing |
| `pselect6` | [🐧](https://man7.org/linux/man-pages/man2/pselect6.2.html) | synchronous I/O multiplexing |
| `poll` | [🐡](https://man.openbsd.org/poll.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=poll&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/poll.2.html) | synchronous I/O multiplexing |
| `ppoll` | [🐡](https://man.openbsd.org/ppoll.2), [🐧](https://man7.org/linux/man-pages/man2/ppoll.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=ppoll&sektion=2) | synchronous I/O multiplexing |
| `kqueue` | [🐡](https://man.openbsd.org/kqueue.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=kqueue&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/kqueue.2.html) | create a new kernel event queue |
| `kqueue1` | [🐡](https://man.openbsd.org/kqueue1.2) | create a new kernel event queue |
| `kevent` | [🐡](https://man.openbsd.org/kevent.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=kevent&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/kevent.2.html) | register events with the kernel event queue |
| `epoll_create1` | [🐧](https://man7.org/linux/man-pages/man2/epoll_create1.2.html) | open an [`epoll(7)`](https://man7.org/linux/man-pages/man7/epoll.7.html) file descriptor |
| `epoll_ctl` | [🐧](https://man7.org/linux/man-pages/man2/epoll_ctl.2.html) | control interface for an `epoll(7)` file descriptor |
| `epoll_pwait` | [🐧](https://man7.org/linux/man-pages/man2/epoll_pwait.2.html) | wait for an I/O event on an `epoll(7)` file descriptor |
| `epoll_pwait2` | [🐧](https://man7.org/linux/man-pages/man2/epoll_pwait2.2.html) | wait for an I/O event on an `epoll(7)` file descriptor |

#### POSIX asynchronous I/O

| Name | OS | Description |
|------|----|-------------|
| `aio_read` | [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_read&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/aio_read.2.html) | asynchronous read from a file |
| `aio_write` | [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_write&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/aio_write.2.html) | asynchronous write to a file |
| `aio_fsync` | [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_fsync&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/aio_fsync.2.html) | asynchronous `fsync` |
| `aio_mlock` | [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_mlock&sektion=2) | asynchronous `mlock` |
| `aio_return` | [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_return&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/aio_return.2.html) | retrieve return status of an asynchronous I/O operation |
| `aio_error` | [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_error&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/aio_error.2.html) | retrieve error status of asynchronous I/O operation |
| `aio_suspend` | [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_suspend&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/aio_suspend.2.html) | suspend until asynchronous I/O operations complete or time out |
| `aio_waitcomplete` | [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_waitcomplete&sektion=2) | wait for the next completion of an asynchronous I/O operation |
| `aio_cancel` | [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_cancel&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/aio_cancel.2.html) | cancel an outstanding asynchronous I/O operation |

#### Asynchronous I/O (Linux)

| Name | OS | Description |
|------|----|-------------|
| `io_setup` | [🐧](https://man7.org/linux/man-pages/man2/io_setup.2.html) | create an asynchronous I/O context |
| `io_submit` | [🐧](https://man7.org/linux/man-pages/man2/io_submit.2.html) | submit asynchronous I/O blocks for processing |
| `io_getevents` | [🐧](https://man7.org/linux/man-pages/man2/io_getevents.2.html) | read asynchronous I/O events from the completion queue |
| `io_pgetevents` | [🐧](https://man7.org/linux/man-pages/man2/io_pgetevents.2.html) | read asynchronous I/O events from the completion queue |
| `io_cancel` | [🐧](https://man7.org/linux/man-pages/man2/io_cancel.2.html) | cancel an outstanding asynchronous I/O operation |
| `io_destroy` | [🐧](https://man7.org/linux/man-pages/man2/io_destroy.2.html) | destroy an asynchronous I/O context |

#### io_uring (Linux)

| Name | OS | Description |
|------|----|-------------|
| `io_uring_setup` | [🐧](https://man7.org/linux/man-pages/man2/io_uring_setup.2.html) | setup a context for performing asynchronous I/O |
| `io_uring_register` | [🐧](https://man7.org/linux/man-pages/man2/io_uring_register.2.html) | register files or user buffers for asynchronous I/O |
| `io_uring_enter` | [🐧](https://man7.org/linux/man-pages/man2/io_uring_enter.2.html) | initiate and/or complete asynchronous I/O |

### Security

#### Sandboxing

| Name | OS | Description |
|------|----|-------------|
| `seccomp` | [🐧](https://man7.org/linux/man-pages/man2/seccomp.2.html) | operate on Secure Computing state of the process |
| `revoke` | [🐡](https://man.openbsd.org/revoke.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=revoke&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/revoke.2.html) | revoke file access |
| `pledge` | [🐡](https://man.openbsd.org/pledge.2) | force the current process into a restricted-service operating mode |
| `unveil` | [🐡](https://man.openbsd.org/unveil.2) | unveil parts of a restricted filesystem view |

#### Jails (FreeBSD)
See [Jails and Containers](https://docs.freebsd.org/en/books/handbook/jails/) chapter in the FreeBSD Handbook for more details.

| Name | OS | Description |
|------|----|-------------|
| `jail` | [😈](https://www.freebsd.org/cgi/man.cgi?query=jail&sektion=2) | sets up a jail and locks the current process in it |
| `jail_attach` | [😈](https://www.freebsd.org/cgi/man.cgi?query=jail_attach&sektion=2) | attaches the current process to an existing jail |
| `jail_get` | [😈](https://www.freebsd.org/cgi/man.cgi?query=jail_get&sektion=2) | retrieves jail parameters |
| `jail_set` | [😈](https://www.freebsd.org/cgi/man.cgi?query=jail_set&sektion=2) | creates a new jail or modifies an existing one |
| `jail_remove` | [😈](https://www.freebsd.org/cgi/man.cgi?query=jail_remove&sektion=2) | removes the jail |

#### Capabilities (Linux)
See [`capabilities(7)`](https://man7.org/linux/man-pages/man7/capabilities.7.html) man page for more details.

| Name | OS | Description |
|------|----|-------------|
| `capget` | [🐧](https://man7.org/linux/man-pages/man2/capget.2.html) | get capabilities of thread(s) |
| `capset` | [🐧](https://man7.org/linux/man-pages/man2/capset.2.html) | set capabilities of thread(s) |

#### Namespaces (Linux)
See [`namespaces(7)`](https://man7.org/linux/man-pages/man7/namespaces.7.html) man page for more details.

| Name | OS | Description |
|------|----|-------------|
| `setns` | [🐧](https://man7.org/linux/man-pages/man2/setns.2.html) | reassociate thread with a namespace |
| `unshare` | [🐧](https://man7.org/linux/man-pages/man2/unshare.2.html) | disassociate parts of the process execution context |

#### Landlock (Linux)
See [`landlock(7)`](https://man7.org/linux/man-pages/man7/landlock.7.html) man page for more details.

| Name | OS | Description |
|------|----|-------------|
| `landlock_create_ruleset` | [🐧](https://man7.org/linux/man-pages/man2/landlock_create_ruleset.2.html) | create a new Landlock ruleset |
| `landlock_add_rule` | [🐧](https://man7.org/linux/man-pages/man2/landlock_add_rule.2.html) | add a new Landlock rule to a ruleset |
| `landlock_restrict_self` | [🐧](https://man7.org/linux/man-pages/man2/landlock_restrict_self.2.html) | enforce a Landlock ruleset |

#### Entropy and random

| Name | OS | Description |
|------|----|-------------|
| `getentropy` | [🐡](https://man.openbsd.org/getentropy.2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getentropy.2.html) | get high-quality entropy |
| `getrandom` | [🐧](https://man7.org/linux/man-pages/man2/getrandom.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getrandom&sektion=2) | get high-quality entropy |

#### Kernel Key Retention Service
See [Kernel Key Retention Service](https://www.kernel.org/doc/html/v5.0/security/keys/core.html) documentation for more details.

| Name | OS | Description |
|------|----|-------------|
| `add_key` | [🐧](https://man7.org/linux/man-pages/man2/add_key.2.html) | add a key to the kernel's key management facility |
| `request_key` | [🐧](https://man7.org/linux/man-pages/man2/request_key.2.html) | request a key from the kernel's key management facility |
| `keyctl` | [🐧](https://man7.org/linux/man-pages/man2/keyctl.2.html) | manipulate the kernel's key management facility |

### Interprocess communication

#### System V semaphores

| Name | OS | Description |
|------|----|-------------|
| `semget` | [🐡](https://man.openbsd.org/semget.2), [🐧](https://man7.org/linux/man-pages/man2/semget.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=semget&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/semget.2.html) | retrieve semaphore set |
| `semop` | [🐡](https://man.openbsd.org/semop.2), [🐧](https://man7.org/linux/man-pages/man2/semop.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=semop&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/semop.2.html) | atomic operations on a set of semaphores |
| `semtimedop` | [🐧](https://man7.org/linux/man-pages/man2/semtimedop.2.html) | atomic operations on a set of semaphores (with timeout) |
| `semctl` | [🐧](https://man7.org/linux/man-pages/man2/semctl.2.html), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/semctl.2.html) | control operations on a set of semaphores |
| `__semctl` | [🐡](https://man.openbsd.org/__semctl.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=__semctl&sektion=2) | control operations on a set of semaphores |

#### POSIX message queues

| Name | OS | Description |
|------|----|-------------|
| `msgget` | [🐡](https://man.openbsd.org/msgget.2), [🐧](https://man7.org/linux/man-pages/man2/msgget.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=msgget&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/msgget.2.html) | retrieve message queue |
| `msgrcv` | [🐡](https://man.openbsd.org/msgrcv.2), [🐧](https://man7.org/linux/man-pages/man2/msgrcv.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=msgrcv&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/msgrcv.2.html) | receive a message from a message queue |
| `msgsnd` | [🐡](https://man.openbsd.org/msgsnd.2), [🐧](https://man7.org/linux/man-pages/man2/msgsnd.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=msgsnd&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/msgsnd.2.html) | send a message to a message queue |
| `msgctl` | [🐡](https://man.openbsd.org/msgctl.2), [🐧](https://man7.org/linux/man-pages/man2/msgctl.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=msgctl&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/msgctl.2.html) | control operations on a message queue |
| `mq_open` | [🐧](https://man7.org/linux/man-pages/man2/mq_open.2.html) | open a message queue |
| `mq_timedreceive` | [🐧](https://man7.org/linux/man-pages/man2/mq_timedreceive.2.html) | receive a message from a message queue |
| `mq_timedsend` | [🐧](https://man7.org/linux/man-pages/man2/mq_timedsend.2.html) | send a message to a message queue |
| `mq_notify` | [🐧](https://man7.org/linux/man-pages/man2/mq_notify.2.html) | register for notification when a message is available |
| `mq_getsetattr` | [🐧](https://man7.org/linux/man-pages/man2/mq_getsetattr.2.html) | get/set message queue attributes |
| `mq_unlink` | [🐧](https://man7.org/linux/man-pages/man2/mq_unlink.2.html) | remove a message queue |

#### Shared memory

| Name | OS | Description |
|------|----|-------------|
| `shmat` | [🐡](https://man.openbsd.org/shmat.2), [🐧](https://man7.org/linux/man-pages/man2/shmat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=shmat&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/shmat.2.html) | map shared memory area |
| `shmget` | [🐡](https://man.openbsd.org/shmget.2), [🐧](https://man7.org/linux/man-pages/man2/shmget.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=shmget&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/shmget.2.html) | get shared memory area identifier |
| `shmctl` | [🐡](https://man.openbsd.org/shmctl.2), [🐧](https://man7.org/linux/man-pages/man2/shmctl.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=shmctl&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/shmctl.2.html) | shared memory area control operations |
| `shmdt` | [🐡](https://man.openbsd.org/shmdt.2), [🐧](https://man7.org/linux/man-pages/man2/shmdt.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=shmdt&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/shmdt.2.html) | unmap shared memory area |

### System

#### General

| Name | OS | Description |
|------|----|-------------|
| `reboot` | [🐡](https://man.openbsd.org/reboot.2), [🐧](https://man7.org/linux/man-pages/man2/reboot.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=reboot&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/reboot.2.html) | reboot system or halt processor |
| `sysctl` | [🐡](https://man.openbsd.org/sysctl.2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sysctl.2.html) | manipulate system information |
| `sysinfo` | [🐧](https://man7.org/linux/man-pages/man2/sysinfo.2.html) | return system information |
| `uname` | [🐧](https://man7.org/linux/man-pages/man2/uname.2.html) | get name and information about current kernel |
| `sethostname` | [🐧](https://man7.org/linux/man-pages/man2/sethostname.2.html) | manipulate hostname |
| `setdomainname` | [🐧](https://man7.org/linux/man-pages/man2/setdomainname.2.html) | manipulate NIS domain name |

#### Kernel

##### Kernel module management (Linux)

| Name | OS | Description |
|------|----|-------------|
| `init_module` | [🐧](https://man7.org/linux/man-pages/man2/init_module.2.html) | load a kernel module |
| `finit_module` | [🐧](https://man7.org/linux/man-pages/man2/finit_module.2.html) | load a kernel module (referenced by a file descriptor) |
| `delete_module` | [🐧](https://man7.org/linux/man-pages/man2/delete_module.2.html) | unload a kernel module |

##### kexec (Linux)

| Name | OS | Description |
|------|----|-------------|
| `kexec_load` | [🐧](https://man7.org/linux/man-pages/man2/kexec_load.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=kexec_load&sektion=2) | load a new kernel for later execution |
| `kexec_file_load` | [🐧](https://man7.org/linux/man-pages/man2/kexec_file_load.2.html) | load a new kernel for later execution (referenced by a file descriptor) |

##### Dynamic kernel linker facility (FreeBSD)
See [`kld(4)`](https://man.freebsd.org/cgi/man.cgi?query=kld&sektion=4) man page for more details

| Name | OS | Description |
|------|----|-------------|
| `kldload` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldload&sektion=2) | load KLD file into the kernel |
| `kldunload` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldunload&sektion=2) | unload KLD fileid from the kernel |
| `kldunloadf` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldunloadf&sektion=2) | unload KLD fileid from the kernel |
| `kldsym` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldsym&sektion=2) | look up address by symbol name in a KLD file |
| `kldfind` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldfind&sektion=2) | return the fileid of a KLD file |
| `kldnext` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldnext&sektion=2) | return the fileid of the next KLD file |
| `kldstat` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldstat&sektion=2) | get status of a KLD file |
| `kldfirstmod` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldfirstmod&sektion=2) | return first module ID from the KLD file |

#### System log

| Name | OS | Description |
|------|----|-------------|
| `syslog` | [🐧](https://man7.org/linux/man-pages/man2/syslog.2.html) | read and/or clear kernel message ring buffer |
| `sendsyslog` | [🐡](https://man.openbsd.org/sendsyslog.2) | send a message to `syslogd(8)` daemon |
| `utrace` | [🐡](https://man.openbsd.org/utrace.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=utrace&sektion=2) | insert user record in `ktrace(2)` log |

#### Swap

| Name | OS | Description |
|------|----|-------------|
| `swapctl` | [🐡](https://man.openbsd.org/swapctl.2) | modify swap configuration |
| `swapon` | [🐧](https://man7.org/linux/man-pages/man2/swapon.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=swapon&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/swapon.2.html) | start swapping to a file/device |
| `swapoff` | [🐧](https://man7.org/linux/man-pages/man2/swapoff.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=swapoff&sektion=2) | stop swapping from a file/device |

#### Clock and time functions

| Name | OS | Description |
|------|----|-------------|
| `gettimeofday` | [🐡](https://man.openbsd.org/gettimeofday.2), [🐧](https://man7.org/linux/man-pages/man2/gettimeofday.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=gettimeofday&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/gettimeofday.2.html) | get the time of day |
| `settimeofday` | [🐡](https://man.openbsd.org/settimeofday.2), [🐧](https://man7.org/linux/man-pages/man2/settimeofday.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=settimeofday&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/settimeofday.2.html) | set the time of day |
| `clock_gettime` | [🐡](https://man.openbsd.org/clock_gettime.2), [🐧](https://man7.org/linux/man-pages/man2/clock_gettime.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=clock_gettime&sektion=2) | get the time of a given clock |
| `clock_settime` | [🐡](https://man.openbsd.org/clock_settime.2), [🐧](https://man7.org/linux/man-pages/man2/clock_settime.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=clock_settime&sektion=2) | set the time for a given clock |
| `clock_getres` | [🐡](https://man.openbsd.org/clock_getres.2), [🐧](https://man7.org/linux/man-pages/man2/clock_getres.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=clock_getres&sektion=2) | get the resolution of a given clock |
| `clock_adjtime` | [🐧](https://man7.org/linux/man-pages/man2/clock_adjtime.2.html) | tune a given clock |
| `adjtime` | [🐡](https://man.openbsd.org/adjtime.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=adjtime&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/adjtime.2.html) | tune the system clock |
| `adjtimex` | [🐧](https://man7.org/linux/man-pages/man2/adjtimex.2.html) | tune the system clock |
| `adjfreq` | [🐡](https://man.openbsd.org/adjfreq.2) | correct the rate of the system clock |

#### Quotas and accounting

| Name | OS | Description |
|------|----|-------------|
| `acct` | [🐡](https://man.openbsd.org/acct.2), [🐧](https://man7.org/linux/man-pages/man2/acct.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=acct&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/acct.2.html) | enable or disable process accounting |
| `quotactl` | [🐡](https://man.openbsd.org/quotactl.2), [🐧](https://man7.org/linux/man-pages/man2/quotactl.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=quotactl&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/quotactl.2.html) | manipulate filesystem quotas |

#### Performance, profiling and and eBPF

| Name | OS | Description |
|------|----|-------------|
| `bpf` | [🐧](https://man7.org/linux/man-pages/man2/bpf.2.html) | manipulate extended Berkeley Packet Filters |
| `perf_event_open` | [🐧](https://man7.org/linux/man-pages/man2/perf_event_open.2.html) | set up performance monitoring |
| `profil` | [🐡](https://man.openbsd.org/profil.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=profil&sektion=2) | manipulate program counter profiling of the current process |

### Meta system calls

| Name | OS | Description |
|------|----|-------------|
| `syscall` | [😈](https://www.freebsd.org/cgi/man.cgi?query=syscall&sektion=2), [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/syscall.2.html) | indirect system call |
| `__syscall` | [😈](https://www.freebsd.org/cgi/man.cgi?query=__syscall&sektion=2) | indirect system call |
| `sysarch` | [🐡](https://man.openbsd.org/sysarch.2), [😈](https://www.freebsd.org/cgi/man.cgi?query=sysarch&sektion=2) | architecture-dependent system call |
| `arch_specific_syscall` | [🐧](https://man7.org/linux/man-pages/man2/arch_specific_syscall.2.html) | architecture-dependent system call |
| `restart_syscall` | [🐧](https://man7.org/linux/man-pages/man2/restart_syscall.2.html) | restart a system call after interruption by a stop signal |
