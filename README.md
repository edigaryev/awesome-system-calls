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
| `fork` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fork.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fork&sektion=2), [🐡](https://man.openbsd.org/fork.2) | create a child process |
| `vfork` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/vfork.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=vfork&sektion=2), [🐡](https://man.openbsd.org/vfork.2) | create a child process and block parent |
| `clone` | [🐧](https://man7.org/linux/man-pages/man2/clone.2.html) | create a child process or thread |
| `clone3` | [🐧](https://man7.org/linux/man-pages/man2/clone3.2.html) | create a child process or thread |
| `posix_spawn` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/posix_spawn.2.html) | spawn a process |
| `execve` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/execve.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=execve&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/execve.2.html), [🐡](https://man.openbsd.org/execve.2) | execute a program |
| `execveat` | [🐧](https://man7.org/linux/man-pages/man2/execveat.2.html) | execute a program (relative to a directory file) |
| `wait4` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/wait4.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=wait4&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/wait4.2.html), [🐡](https://man.openbsd.org/wait4.2) | wait for process to change state |
| `waitid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/waitid.2.html), [🐧](https://man7.org/linux/man-pages/man2/waitid.2.html), [🐡](https://man.openbsd.org/waitid.2) | wait for process to change state |
| `exit` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/exit.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=exit&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/exit.2.html), [🐡](https://man.openbsd.org/exit.2) | terminate the calling process |

##### Threads

| Name | OS | Description |
|------|----|-------------|
| `tkill` | [🐧](https://man7.org/linux/man-pages/man2/tkill.2.html) | send a signal to a thread |
| `tgkill` | [🐧](https://man7.org/linux/man-pages/man2/tgkill.2.html) | send a signal to a thread in a thread group |
| `thrkill` | [🐡](https://man.openbsd.org/thrkill.2) | send signal to a thread |
| `exit_group` | [🐧](https://man7.org/linux/man-pages/man2/exit_group.2.html) | exit all threads in a process's thread group |
| `set_tid_address` | [🐧](https://man7.org/linux/man-pages/man2/set_tid_address.2.html) | set pointer to thread ID |
| `__get_tcb` | [🐡](https://man.openbsd.org/__get_tcb.2) | get the address of the thread control block of the current thread |
| `__set_tcb` | [🐡](https://man.openbsd.org/__set_tcb.2) | set the address of the thread control block of the current thread |
| `__tfork` | [🐡](https://man.openbsd.org/__tfork.2) | create a new kernel thread in the current process |
| `__thrsleep` | [🐡](https://man.openbsd.org/__thrsleep.2) | userspace thread sleep |
| `__thrwakeup` | [🐡](https://man.openbsd.org/__thrwakeup.2) | userspace thread wakeup |

#### File system relationships

| Name | OS | Description |
|------|----|-------------|
| `getcwd` | [🐧](https://man7.org/linux/man-pages/man2/getcwd.2.html) | get current working directory |
| `__getcwd` | [😈](https://www.freebsd.org/cgi/man.cgi?query=__getcwd&sektion=2), [🐡](https://man.openbsd.org/__getcwd.2) | get current working directory |
| `chdir` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/chdir.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=chdir&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/chdir.2.html), [🐡](https://man.openbsd.org/chdir.2) | set current working directory |
| `fchdir` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fchdir.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fchdir&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/fchdir.2.html), [🐡](https://man.openbsd.org/fchdir.2) | set current working directory (referenced by a file descriptor) |
| `chroot` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/chroot.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=chroot&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/chroot.2.html), [🐡](https://man.openbsd.org/chroot.2) | change root directory |
| `pivot_root` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pivot_root.2.html), [🐧](https://man7.org/linux/man-pages/man2/pivot_root.2.html) | change root mount |
| `setfsuid` | [🐧](https://man7.org/linux/man-pages/man2/setfsuid.2.html) | set user identity used for filesystem checks |
| `setfsgid` | [🐧](https://man7.org/linux/man-pages/man2/setfsgid.2.html) | set group identity used for filesystem checks |

#### Credentials

##### UID

| Name | OS | Description |
|------|----|-------------|
| `getuid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getuid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getuid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getuid.2.html), [🐡](https://man.openbsd.org/getuid.2) |  |
| `setuid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setuid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setuid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/setuid.2.html), [🐡](https://man.openbsd.org/setuid.2) |  |
| `geteuid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/geteuid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=geteuid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/geteuid.2.html), [🐡](https://man.openbsd.org/geteuid.2) |  |
| `seteuid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/seteuid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=seteuid&sektion=2), [🐡](https://man.openbsd.org/seteuid.2) |  |
| `getresuid` | [😈](https://www.freebsd.org/cgi/man.cgi?query=getresuid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getresuid.2.html), [🐡](https://man.openbsd.org/getresuid.2) |  |
| `setresuid` | [😈](https://www.freebsd.org/cgi/man.cgi?query=setresuid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/setresuid.2.html), [🐡](https://man.openbsd.org/setresuid.2) |  |
| `setreuid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setreuid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setreuid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/setreuid.2.html), [🐡](https://man.openbsd.org/setreuid.2) |  |

##### GID

| Name | OS | Description |
|------|----|-------------|
| `getgid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getgid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getgid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getgid.2.html), [🐡](https://man.openbsd.org/getgid.2) |  |
| `setgid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setgid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setgid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/setgid.2.html), [🐡](https://man.openbsd.org/setgid.2) |  |
| `getegid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getegid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getegid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getegid.2.html), [🐡](https://man.openbsd.org/getegid.2) |  |
| `setegid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setegid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setegid&sektion=2), [🐡](https://man.openbsd.org/setegid.2) |  |
| `getresgid` | [😈](https://www.freebsd.org/cgi/man.cgi?query=getresgid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getresgid.2.html), [🐡](https://man.openbsd.org/getresgid.2) |  |
| `setresgid` | [😈](https://www.freebsd.org/cgi/man.cgi?query=setresgid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/setresgid.2.html), [🐡](https://man.openbsd.org/setresgid.2) |  |
| `setregid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setregid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setregid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/setregid.2.html), [🐡](https://man.openbsd.org/setregid.2) |  |
| `getgroups` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getgroups.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getgroups&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getgroups.2.html), [🐡](https://man.openbsd.org/getgroups.2) |  |
| `setgroups` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setgroups.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setgroups&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/setgroups.2.html), [🐡](https://man.openbsd.org/setgroups.2) |  |

##### Process groups

| Name | OS | Description |
|------|----|-------------|
| `getpgid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getpgid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getpgid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getpgid.2.html), [🐡](https://man.openbsd.org/getpgid.2) |  |
| `setpgid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setpgid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setpgid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/setpgid.2.html), [🐡](https://man.openbsd.org/setpgid.2) |  |
| `getpgrp` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getpgrp.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getpgrp&sektion=2), [🐡](https://man.openbsd.org/getpgrp.2) |  |

#### Signals

| Name | OS | Description |
|------|----|-------------|
| `kill` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/kill.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=kill&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/kill.2.html), [🐡](https://man.openbsd.org/kill.2) |  |
| `sigaction` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sigaction.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sigaction&sektion=2), [🐡](https://man.openbsd.org/sigaction.2) |  |
| `rt_sigaction` | [🐧](https://man7.org/linux/man-pages/man2/rt_sigaction.2.html) |  |
| `sigaltstack` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sigaltstack.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sigaltstack&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/sigaltstack.2.html), [🐡](https://man.openbsd.org/sigaltstack.2) |  |
| `sigpending` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sigpending.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sigpending&sektion=2), [🐡](https://man.openbsd.org/sigpending.2) |  |
| `rt_sigpending` | [🐧](https://man7.org/linux/man-pages/man2/rt_sigpending.2.html) |  |
| `sigprocmask` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sigprocmask.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sigprocmask&sektion=2), [🐡](https://man.openbsd.org/sigprocmask.2) |  |
| `rt_sigprocmask` | [🐧](https://man7.org/linux/man-pages/man2/rt_sigprocmask.2.html) |  |
| `sigreturn` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sigreturn.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sigreturn&sektion=2), [🐡](https://man.openbsd.org/sigreturn.2) |  |
| `rt_sigreturn` | [🐧](https://man7.org/linux/man-pages/man2/rt_sigreturn.2.html) |  |
| `sigsuspend` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sigsuspend.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sigsuspend&sektion=2), [🐡](https://man.openbsd.org/sigsuspend.2) |  |
| `rt_sigsuspend` | [🐧](https://man7.org/linux/man-pages/man2/rt_sigsuspend.2.html) |  |
| `rt_sigtimedwait` | [🐧](https://man7.org/linux/man-pages/man2/rt_sigtimedwait.2.html) |  |
| `rt_sigqueueinfo` | [🐧](https://man7.org/linux/man-pages/man2/rt_sigqueueinfo.2.html) |  |
| `rt_tgsigqueueinfo` | [🐧](https://man7.org/linux/man-pages/man2/rt_tgsigqueueinfo.2.html) |  |
| `__thrsigdivert` | [🐡](https://man.openbsd.org/__thrsigdivert.2) |  |

#### Virtual memory

| Name | OS | Description |
|------|----|-------------|
| `brk` | [🐧](https://man7.org/linux/man-pages/man2/brk.2.html) |  |
| `break` | [😈](https://www.freebsd.org/cgi/man.cgi?query=break&sektion=2), [🐡](https://man.openbsd.org/break.2) |  |
| `kbind` | [🐡](https://man.openbsd.org/kbind.2) |  |
| `minherit` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/minherit.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=minherit&sektion=2), [🐡](https://man.openbsd.org/minherit.2) |  |
| `mlock` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mlock.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mlock&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/mlock.2.html), [🐡](https://man.openbsd.org/mlock.2) |  |
| `mlockall` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mlockall.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mlockall&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/mlockall.2.html), [🐡](https://man.openbsd.org/mlockall.2) |  |
| `mmap` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mmap.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mmap&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/mmap.2.html), [🐡](https://man.openbsd.org/mmap.2) |  |
| `remap_file_pages` | [🐧](https://man7.org/linux/man-pages/man2/remap_file_pages.2.html) |  |
| `mprotect` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mprotect.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mprotect&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/mprotect.2.html), [🐡](https://man.openbsd.org/mprotect.2) |  |
| `mimmutable` | [🐡](https://man.openbsd.org/mimmutable.2) |  |
| `msync` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/msync.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=msync&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/msync.2.html), [🐡](https://man.openbsd.org/msync.2) |  |
| `munlock` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/munlock.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=munlock&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/munlock.2.html), [🐡](https://man.openbsd.org/munlock.2) |  |
| `munlockall` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/munlockall.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=munlockall&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/munlockall.2.html), [🐡](https://man.openbsd.org/munlockall.2) |  |
| `munmap` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/munmap.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=munmap&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/munmap.2.html), [🐡](https://man.openbsd.org/munmap.2) |  |
| `madvise` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/madvise.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=madvise&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/madvise.2.html), [🐡](https://man.openbsd.org/madvise.2) |  |
| `process_madvise` | [🐧](https://man7.org/linux/man-pages/man2/process_madvise.2.html) |  |
| `mquery` | [🐡](https://man.openbsd.org/mquery.2) |  |
| `mincore` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mincore.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mincore&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/mincore.2.html) |  |
| `mlock2` | [🐧](https://man7.org/linux/man-pages/man2/mlock2.2.html) |  |
| `mmap2` | [🐧](https://man7.org/linux/man-pages/man2/mmap2.2.html) |  |
| `mbind` | [🐧](https://man7.org/linux/man-pages/man2/mbind.2.html) |  |
| `mremap` | [🐧](https://man7.org/linux/man-pages/man2/mremap.2.html) |  |
| `pkey_mprotect` | [🐧](https://man7.org/linux/man-pages/man2/pkey_mprotect.2.html) |  |
| `pkey_alloc` | [🐧](https://man7.org/linux/man-pages/man2/pkey_alloc.2.html) |  |
| `pkey_free` | [🐧](https://man7.org/linux/man-pages/man2/pkey_free.2.html) |  |

#### Timers

| Name | OS | Description |
|------|----|-------------|
| `getitimer` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getitimer.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getitimer&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getitimer.2.html), [🐡](https://man.openbsd.org/getitimer.2) |  |
| `setitimer` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setitimer.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setitimer&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/setitimer.2.html), [🐡](https://man.openbsd.org/setitimer.2) |  |
| `timer_create` | [🐧](https://man7.org/linux/man-pages/man2/timer_create.2.html) |  |
| `timer_delete` | [🐧](https://man7.org/linux/man-pages/man2/timer_delete.2.html) |  |
| `timerfd_gettime` | [🐧](https://man7.org/linux/man-pages/man2/timerfd_gettime.2.html) |  |
| `timerfd_settime` | [🐧](https://man7.org/linux/man-pages/man2/timerfd_settime.2.html) |  |
| `timer_getoverrun` | [🐧](https://man7.org/linux/man-pages/man2/timer_getoverrun.2.html) |  |
| `timer_gettime` | [🐧](https://man7.org/linux/man-pages/man2/timer_gettime.2.html) |  |
| `timer_settime` | [🐧](https://man7.org/linux/man-pages/man2/timer_settime.2.html) |  |

#### Sessions

| Name | OS | Description |
|------|----|-------------|
| `getlogin_r` | [🐡](https://man.openbsd.org/getlogin_r.2) |  |
| `getsid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getsid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getsid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getsid.2.html), [🐡](https://man.openbsd.org/getsid.2) |  |
| `setsid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setsid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setsid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/setsid.2.html), [🐡](https://man.openbsd.org/setsid.2) |  |
| `setlogin` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setlogin.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setlogin&sektion=2), [🐡](https://man.openbsd.org/setlogin.2) |  |

#### Metadata

##### Process

| Name | OS | Description |
|------|----|-------------|
| `getpid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getpid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getpid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getpid.2.html), [🐡](https://man.openbsd.org/getpid.2) |  |
| `getppid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getppid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getppid&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getppid.2.html), [🐡](https://man.openbsd.org/getppid.2) |  |

##### Thread

| Name | OS | Description |
|------|----|-------------|
| `getthrid` | [🐡](https://man.openbsd.org/getthrid.2) |  |
| `gettid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/gettid.2.html), [🐧](https://man7.org/linux/man-pages/man2/gettid.2.html) |  |
| `getthrname` | [🐡](https://man.openbsd.org/getthrname.2) |  |
| `setthrname` | [🐡](https://man.openbsd.org/setthrname.2) |  |

##### Other

| Name | OS | Description |
|------|----|-------------|
| `issetugid` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/issetugid.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=issetugid&sektion=2), [🐡](https://man.openbsd.org/issetugid.2) |  |
| `personality` | [🐧](https://man7.org/linux/man-pages/man2/personality.2.html) |  |

#### CPU and NUMA

| Name | OS | Description |
|------|----|-------------|
| `getcpu` | [🐧](https://man7.org/linux/man-pages/man2/getcpu.2.html) |  |
| `get_mempolicy` | [🐧](https://man7.org/linux/man-pages/man2/get_mempolicy.2.html) |  |
| `set_mempolicy` | [🐧](https://man7.org/linux/man-pages/man2/set_mempolicy.2.html) |  |
| `migrate_pages` | [🐧](https://man7.org/linux/man-pages/man2/migrate_pages.2.html) |  |
| `move_pages` | [🐧](https://man7.org/linux/man-pages/man2/move_pages.2.html) |  |

#### Scheduling

| Name | OS | Description |
|------|----|-------------|
| `getpriority` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getpriority.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getpriority&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getpriority.2.html), [🐡](https://man.openbsd.org/getpriority.2) |  |
| `setpriority` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setpriority.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setpriority&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/setpriority.2.html), [🐡](https://man.openbsd.org/setpriority.2) |  |
| `sched_yield` | [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_yield&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/sched_yield.2.html), [🐡](https://man.openbsd.org/sched_yield.2) |  |
| `sched_getparam` | [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_getparam&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/sched_getparam.2.html) |  |
| `sched_setparam` | [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_setparam&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/sched_setparam.2.html) |  |
| `sched_get_priority_min` | [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_get_priority_min&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/sched_get_priority_min.2.html) |  |
| `sched_get_priority_max` | [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_get_priority_max&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/sched_get_priority_max.2.html) |  |
| `sched_getscheduler` | [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_getscheduler&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/sched_getscheduler.2.html) |  |
| `sched_setscheduler` | [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_setscheduler&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/sched_setscheduler.2.html) |  |
| `sched_rr_get_interval` | [😈](https://www.freebsd.org/cgi/man.cgi?query=sched_rr_get_interval&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/sched_rr_get_interval.2.html) |  |
| `sched_getattr` | [🐧](https://man7.org/linux/man-pages/man2/sched_getattr.2.html) |  |
| `sched_setattr` | [🐧](https://man7.org/linux/man-pages/man2/sched_setattr.2.html) |  |
| `sched_getaffinity` | [🐧](https://man7.org/linux/man-pages/man2/sched_getaffinity.2.html) |  |
| `sched_setaffinity` | [🐧](https://man7.org/linux/man-pages/man2/sched_setaffinity.2.html) |  |
| `ioprio_get` | [🐧](https://man7.org/linux/man-pages/man2/ioprio_get.2.html) |  |
| `ioprio_set` | [🐧](https://man7.org/linux/man-pages/man2/ioprio_set.2.html) |  |

#### Resource limits and utilization

| Name | OS | Description |
|------|----|-------------|
| `getrlimit` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getrlimit.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getrlimit&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getrlimit.2.html), [🐡](https://man.openbsd.org/getrlimit.2) |  |
| `setrlimit` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setrlimit.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setrlimit&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/setrlimit.2.html), [🐡](https://man.openbsd.org/setrlimit.2) |  |
| `prlimit64` | [🐧](https://man7.org/linux/man-pages/man2/prlimit64.2.html) |  |
| `getrusage` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getrusage.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getrusage&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getrusage.2.html), [🐡](https://man.openbsd.org/getrusage.2) |  |
| `times` | [🐧](https://man7.org/linux/man-pages/man2/times.2.html) |  |

#### Terminal

| Name | OS | Description |
|------|----|-------------|
| `vhangup` | [🐧](https://man7.org/linux/man-pages/man2/vhangup.2.html) |  |

#### Network routing

| Name | OS | Description |
|------|----|-------------|
| `getrtable` | [🐡](https://man.openbsd.org/getrtable.2) |  |
| `setrtable` | [🐡](https://man.openbsd.org/setrtable.2) |  |

#### Sleeping

| Name | OS | Description |
|------|----|-------------|
| `nanosleep` | [😈](https://www.freebsd.org/cgi/man.cgi?query=nanosleep&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/nanosleep.2.html), [🐡](https://man.openbsd.org/nanosleep.2) |  |
| `clock_nanosleep` | [😈](https://www.freebsd.org/cgi/man.cgi?query=clock_nanosleep&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/clock_nanosleep.2.html) |  |

#### Locking and synchronization

| Name | OS | Description |
|------|----|-------------|
| `futex` | [🐧](https://man7.org/linux/man-pages/man2/futex.2.html), [🐡](https://man.openbsd.org/futex.2) |  |
| `futex_waitv` | [🐧](https://man7.org/linux/man-pages/man2/futex_waitv.2.html) |  |
| `membarrier` | [🐧](https://man7.org/linux/man-pages/man2/membarrier.2.html) |  |
| `get_robust_list` | [🐧](https://man7.org/linux/man-pages/man2/get_robust_list.2.html) |  |
| `set_robust_list` | [🐧](https://man7.org/linux/man-pages/man2/set_robust_list.2.html) |  |

#### Operations on other processes

##### Tracing

| Name | OS | Description |
|------|----|-------------|
| `ktrace` | [😈](https://www.freebsd.org/cgi/man.cgi?query=ktrace&sektion=2), [🐡](https://man.openbsd.org/ktrace.2) |  |
| `ptrace` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/ptrace.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=ptrace&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/ptrace.2.html), [🐡](https://man.openbsd.org/ptrace.2) |  |

##### Virtual memory

| Name | OS | Description |
|------|----|-------------|
| `process_vm_readv` | [🐧](https://man7.org/linux/man-pages/man2/process_vm_readv.2.html) |  |
| `process_vm_writev` | [🐧](https://man7.org/linux/man-pages/man2/process_vm_writev.2.html) |  |

##### Other

| Name | OS | Description |
|------|----|-------------|
| `prctl` | [🐧](https://man7.org/linux/man-pages/man2/prctl.2.html) |  |
| `kcmp` | [🐧](https://man7.org/linux/man-pages/man2/kcmp.2.html) |  |

### File descriptors and handles

#### Open or create a file

| Name | OS | Description |
|------|----|-------------|
| `open` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/open.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=open&sektion=2), [🐡](https://man.openbsd.org/open.2) |  |
| `openat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/openat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=openat&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/openat.2.html), [🐡](https://man.openbsd.org/openat.2) |  |
| `openat2` | [🐧](https://man7.org/linux/man-pages/man2/openat2.2.html) |  |
| `open_by_handle_at` | [🐧](https://man7.org/linux/man-pages/man2/open_by_handle_at.2.html) |  |
| `name_to_handle_at` | [🐧](https://man7.org/linux/man-pages/man2/name_to_handle_at.2.html) |  |
| `getfh` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getfh.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getfh&sektion=2), [🐡](https://man.openbsd.org/getfh.2) |  |
| `fhopen` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fhopen.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fhopen&sektion=2), [🐡](https://man.openbsd.org/fhopen.2) |  |

#### Create a pipe

| Name | OS | Description |
|------|----|-------------|
| `pipe` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pipe.2.html), [🐡](https://man.openbsd.org/pipe.2) |  |
| `pipe2` | [😈](https://www.freebsd.org/cgi/man.cgi?query=pipe2&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/pipe2.2.html), [🐡](https://man.openbsd.org/pipe2.2) |  |

#### Open or create something else

| Name | OS | Description |
|------|----|-------------|
| `eventfd2` | [🐧](https://man7.org/linux/man-pages/man2/eventfd2.2.html) |  |
| `memfd_create` | [🐧](https://man7.org/linux/man-pages/man2/memfd_create.2.html) |  |
| `memfd_secret` | [🐧](https://man7.org/linux/man-pages/man2/memfd_secret.2.html) |  |
| `timerfd_create` | [🐧](https://man7.org/linux/man-pages/man2/timerfd_create.2.html) |  |
| `userfaultfd` | [🐧](https://man7.org/linux/man-pages/man2/userfaultfd.2.html) |  |
| `signalfd4` | [🐧](https://man7.org/linux/man-pages/man2/signalfd4.2.html) |  |
| `pidfd_open` | [🐧](https://man7.org/linux/man-pages/man2/pidfd_open.2.html) |  |

#### Duplicate a file descriptor

| Name | OS | Description |
|------|----|-------------|
| `dup` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/dup.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=dup&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/dup.2.html), [🐡](https://man.openbsd.org/dup.2) |  |
| `dup2` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/dup2.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=dup2&sektion=2), [🐡](https://man.openbsd.org/dup2.2) |  |
| `dup3` | [🐧](https://man7.org/linux/man-pages/man2/dup3.2.html), [🐡](https://man.openbsd.org/dup3.2) |  |

#### Modify file descriptor metadata

| Name | OS | Description |
|------|----|-------------|
| `fcntl` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fcntl.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fcntl&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/fcntl.2.html), [🐡](https://man.openbsd.org/fcntl.2) |  |
| `getdtablecount` | [🐡](https://man.openbsd.org/getdtablecount.2) |  |
| `ioctl` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/ioctl.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=ioctl&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/ioctl.2.html), [🐡](https://man.openbsd.org/ioctl.2) |  |
| `lseek` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/lseek.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=lseek&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/lseek.2.html), [🐡](https://man.openbsd.org/lseek.2) |  |
| `llseek` | [🐧](https://man7.org/linux/man-pages/man2/llseek.2.html) |  |

#### Provide file descriptor hints

| Name | OS | Description |
|------|----|-------------|
| `fadvise64` | [🐧](https://man7.org/linux/man-pages/man2/fadvise64.2.html) |  |
| `fadvise64_64` | [🐧](https://man7.org/linux/man-pages/man2/fadvise64_64.2.html) |  |
| `readahead` | [🐧](https://man7.org/linux/man-pages/man2/readahead.2.html) |  |

#### I/O on a file descriptor

##### Read

| Name | OS | Description |
|------|----|-------------|
| `read` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/read.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=read&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/read.2.html), [🐡](https://man.openbsd.org/read.2) |  |
| `readv` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/readv.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=readv&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/readv.2.html), [🐡](https://man.openbsd.org/readv.2) |  |
| `pread` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pread.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=pread&sektion=2), [🐡](https://man.openbsd.org/pread.2) |  |
| `preadv` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/preadv.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=preadv&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/preadv.2.html), [🐡](https://man.openbsd.org/preadv.2) |  |
| `preadv2` | [🐧](https://man7.org/linux/man-pages/man2/preadv2.2.html) |  |

##### Write

| Name | OS | Description |
|------|----|-------------|
| `write` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/write.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=write&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/write.2.html), [🐡](https://man.openbsd.org/write.2) |  |
| `writev` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/writev.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=writev&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/writev.2.html), [🐡](https://man.openbsd.org/writev.2) |  |
| `pwrite` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pwrite.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=pwrite&sektion=2), [🐡](https://man.openbsd.org/pwrite.2) |  |
| `pwritev` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pwritev.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=pwritev&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/pwritev.2.html), [🐡](https://man.openbsd.org/pwritev.2) |  |
| `pwritev2` | [🐧](https://man7.org/linux/man-pages/man2/pwritev2.2.html) |  |

##### Zero-copy

| Name | OS | Description |
|------|----|-------------|
| `copy_file_range` | [😈](https://www.freebsd.org/cgi/man.cgi?query=copy_file_range&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/copy_file_range.2.html) |  |
| `sendfile` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sendfile.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sendfile&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/sendfile.2.html) |  |
| `splice` | [🐧](https://man7.org/linux/man-pages/man2/splice.2.html) |  |
| `tee` | [🐧](https://man7.org/linux/man-pages/man2/tee.2.html) |  |
| `fallocate` | [🐧](https://man7.org/linux/man-pages/man2/fallocate.2.html) |  |
| `vmsplice` | [🐧](https://man7.org/linux/man-pages/man2/vmsplice.2.html) |  |

#### pidfd (Linux)

| Name | OS | Description |
|------|----|-------------|
| `pidfd_getfd` | [🐧](https://man7.org/linux/man-pages/man2/pidfd_getfd.2.html) |  |
| `pidfd_send_signal` | [🐧](https://man7.org/linux/man-pages/man2/pidfd_send_signal.2.html) |  |

#### Close a file descriptor

| Name | OS | Description |
|------|----|-------------|
| `close` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/close.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=close&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/close.2.html), [🐡](https://man.openbsd.org/close.2) | delete a descriptor |
| `closefrom` | [🐡](https://man.openbsd.org/closefrom.2) | delete many descriptors |
| `close_range` | [😈](https://www.freebsd.org/cgi/man.cgi?query=close_range&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/close_range.2.html) |  |

### File system

#### Create an object

| Name | OS | Description |
|------|----|-------------|
| `mkdir` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mkdir.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mkdir&sektion=2), [🐡](https://man.openbsd.org/mkdir.2) |  |
| `mkfifo` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mkfifo.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mkfifo&sektion=2), [🐡](https://man.openbsd.org/mkfifo.2) |  |
| `mknod` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mknod.2.html), [🐡](https://man.openbsd.org/mknod.2) |  |
| `link` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/link.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=link&sektion=2), [🐡](https://man.openbsd.org/link.2) |  |
| `symlink` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/symlink.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=symlink&sektion=2), [🐡](https://man.openbsd.org/symlink.2) |  |
| `symlinkat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/symlinkat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=symlinkat&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/symlinkat.2.html), [🐡](https://man.openbsd.org/symlinkat.2) |  |
| `linkat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/linkat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=linkat&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/linkat.2.html), [🐡](https://man.openbsd.org/linkat.2) |  |
| `mkdirat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mkdirat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mkdirat&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/mkdirat.2.html), [🐡](https://man.openbsd.org/mkdirat.2) |  |
| `mkfifoat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mkfifoat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mkfifoat&sektion=2), [🐡](https://man.openbsd.org/mkfifoat.2) |  |
| `mknodat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mknodat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mknodat&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/mknodat.2.html), [🐡](https://man.openbsd.org/mknodat.2) |  |

#### Modify an objects

| Name | OS | Description |
|------|----|-------------|
| `rename` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/rename.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=rename&sektion=2), [🐡](https://man.openbsd.org/rename.2) |  |
| `renameat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/renameat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=renameat&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/renameat.2.html), [🐡](https://man.openbsd.org/renameat.2) |  |
| `renameat2` | [🐧](https://man7.org/linux/man-pages/man2/renameat2.2.html) |  |
| `truncate` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/truncate.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=truncate&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/truncate.2.html), [🐡](https://man.openbsd.org/truncate.2) |  |
| `ftruncate` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/ftruncate.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=ftruncate&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/ftruncate.2.html), [🐡](https://man.openbsd.org/ftruncate.2) |  |

#### Change object permissions

| Name | OS | Description |
|------|----|-------------|
| `access` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/access.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=access&sektion=2), [🐡](https://man.openbsd.org/access.2) |  |
| `faccessat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/faccessat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=faccessat&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/faccessat.2.html), [🐡](https://man.openbsd.org/faccessat.2) |  |
| `faccessat2` | [🐧](https://man7.org/linux/man-pages/man2/faccessat2.2.html) |  |
| `chflags` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/chflags.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=chflags&sektion=2), [🐡](https://man.openbsd.org/chflags.2) |  |
| `chmod` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/chmod.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=chmod&sektion=2), [🐡](https://man.openbsd.org/chmod.2) |  |
| `chown` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/chown.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=chown&sektion=2), [🐡](https://man.openbsd.org/chown.2) |  |
| `lchown` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/lchown.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=lchown&sektion=2), [🐡](https://man.openbsd.org/lchown.2) |  |
| `umask` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/umask.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=umask&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/umask.2.html), [🐡](https://man.openbsd.org/umask.2) |  |
| `chflagsat` | [😈](https://www.freebsd.org/cgi/man.cgi?query=chflagsat&sektion=2), [🐡](https://man.openbsd.org/chflagsat.2) |  |
| `fchmodat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fchmodat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fchmodat&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/fchmodat.2.html), [🐡](https://man.openbsd.org/fchmodat.2) |  |
| `fchmodat2` | [🐧](https://man7.org/linux/man-pages/man2/fchmodat2.2.html) |  |
| `fchownat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fchownat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fchownat&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/fchownat.2.html), [🐡](https://man.openbsd.org/fchownat.2) |  |
| `fchflags` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fchflags.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fchflags&sektion=2), [🐡](https://man.openbsd.org/fchflags.2) |  |
| `fchmod` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fchmod.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fchmod&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/fchmod.2.html), [🐡](https://man.openbsd.org/fchmod.2) |  |
| `fchown` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fchown.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fchown&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/fchown.2.html), [🐡](https://man.openbsd.org/fchown.2) |  |

#### Retrieve object stats

| Name | OS | Description |
|------|----|-------------|
| `lstat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/lstat.2.html), [🐧](https://man7.org/linux/man-pages/man2/lstat.2.html), [🐡](https://man.openbsd.org/lstat.2) |  |
| `fhstat` | [😈](https://www.freebsd.org/cgi/man.cgi?query=fhstat&sektion=2), [🐡](https://man.openbsd.org/fhstat.2) |  |
| `fhstatfs` | [😈](https://www.freebsd.org/cgi/man.cgi?query=fhstatfs&sektion=2), [🐡](https://man.openbsd.org/fhstatfs.2) |  |
| `readlink` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/readlink.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=readlink&sektion=2), [🐡](https://man.openbsd.org/readlink.2) |  |
| `__realpath` | [🐡](https://man.openbsd.org/__realpath.2) |  |
| `stat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/stat.2.html), [🐧](https://man7.org/linux/man-pages/man2/stat.2.html), [🐡](https://man.openbsd.org/stat.2) |  |
| `statx` | [🐧](https://man7.org/linux/man-pages/man2/statx.2.html) |  |
| `fstatat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fstatat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fstatat&sektion=2), [🐡](https://man.openbsd.org/fstatat.2) |  |
| `statfs` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/statfs.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=statfs&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/statfs.2.html), [🐡](https://man.openbsd.org/statfs.2) |  |
| `newfstatat` | [🐧](https://man7.org/linux/man-pages/man2/newfstatat.2.html) |  |
| `readlinkat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/readlinkat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=readlinkat&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/readlinkat.2.html), [🐡](https://man.openbsd.org/readlinkat.2) |  |
| `utimes` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/utimes.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=utimes&sektion=2), [🐡](https://man.openbsd.org/utimes.2) |  |
| `lutimes` | [😈](https://www.freebsd.org/cgi/man.cgi?query=lutimes&sektion=2) |  |

#### Remove object

| Name | OS | Description |
|------|----|-------------|
| `unlink` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/unlink.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=unlink&sektion=2), [🐡](https://man.openbsd.org/unlink.2) |  |
| `unlinkat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/unlinkat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=unlinkat&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/unlinkat.2.html), [🐡](https://man.openbsd.org/unlinkat.2) |  |
| `rmdir` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/rmdir.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=rmdir&sektion=2), [🐡](https://man.openbsd.org/rmdir.2) |  |
| `utimensat` | [😈](https://www.freebsd.org/cgi/man.cgi?query=utimensat&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/utimensat.2.html), [🐡](https://man.openbsd.org/utimensat.2) |  |
| `flock` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/flock.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=flock&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/flock.2.html), [🐡](https://man.openbsd.org/flock.2) |  |
| `fstat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fstat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fstat&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/fstat.2.html), [🐡](https://man.openbsd.org/fstat.2) |  |
| `futimens` | [😈](https://www.freebsd.org/cgi/man.cgi?query=futimens&sektion=2), [🐡](https://man.openbsd.org/futimens.2) |  |
| `futimes` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/futimes.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=futimes&sektion=2), [🐡](https://man.openbsd.org/futimes.2) |  |
| `getdents` | [🐡](https://man.openbsd.org/getdents.2) |  |

#### Change mount points

| Name | OS | Description |
|------|----|-------------|
| `mount` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mount.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=mount&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/mount.2.html), [🐡](https://man.openbsd.org/mount.2) |  |
| `getfsstat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getfsstat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getfsstat&sektion=2), [🐡](https://man.openbsd.org/getfsstat.2) |  |
| `fstatfs` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fstatfs.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fstatfs&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/fstatfs.2.html), [🐡](https://man.openbsd.org/fstatfs.2) |  |
| `unmount` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/unmount.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=unmount&sektion=2), [🐡](https://man.openbsd.org/unmount.2) |  |
| `umount2` | [🐧](https://man7.org/linux/man-pages/man2/umount2.2.html) |  |

#### Change global file system state

| Name | OS | Description |
|------|----|-------------|
| `sync` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sync.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sync&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/sync.2.html), [🐡](https://man.openbsd.org/sync.2) |  |
| `fsync` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fsync.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fsync&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/fsync.2.html), [🐡](https://man.openbsd.org/fsync.2) |  |
| `fdatasync` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fdatasync.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fdatasync&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/fdatasync.2.html) |  |
| `sync_file_range` | [🐧](https://man7.org/linux/man-pages/man2/sync_file_range.2.html) |  |
| `sync_file_range2` | [🐧](https://man7.org/linux/man-pages/man2/sync_file_range2.2.html) |  |
| `syncfs` | [🐧](https://man7.org/linux/man-pages/man2/syncfs.2.html) |  |

#### Change object extended attributes

| Name | OS | Description |
|------|----|-------------|
| `fgetxattr` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fgetxattr.2.html), [🐧](https://man7.org/linux/man-pages/man2/fgetxattr.2.html) |  |
| `flistxattr` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/flistxattr.2.html), [🐧](https://man7.org/linux/man-pages/man2/flistxattr.2.html) |  |
| `fremovexattr` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fremovexattr.2.html), [🐧](https://man7.org/linux/man-pages/man2/fremovexattr.2.html) |  |
| `fsetxattr` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fsetxattr.2.html), [🐧](https://man7.org/linux/man-pages/man2/fsetxattr.2.html) |  |
| `getxattr` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getxattr.2.html), [🐧](https://man7.org/linux/man-pages/man2/getxattr.2.html) |  |
| `lgetxattr` | [🐧](https://man7.org/linux/man-pages/man2/lgetxattr.2.html) |  |
| `listxattr` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/listxattr.2.html), [🐧](https://man7.org/linux/man-pages/man2/listxattr.2.html) |  |
| `llistxattr` | [🐧](https://man7.org/linux/man-pages/man2/llistxattr.2.html) |  |
| `lremovexattr` | [🐧](https://man7.org/linux/man-pages/man2/lremovexattr.2.html) |  |
| `lsetxattr` | [🐧](https://man7.org/linux/man-pages/man2/lsetxattr.2.html) |  |
| `removexattr` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/removexattr.2.html), [🐧](https://man7.org/linux/man-pages/man2/removexattr.2.html) |  |
| `setxattr` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setxattr.2.html), [🐧](https://man7.org/linux/man-pages/man2/setxattr.2.html) |  |

#### Watch objects

| Name | OS | Description |
|------|----|-------------|
| `inotify_add_watch` | [🐧](https://man7.org/linux/man-pages/man2/inotify_add_watch.2.html) |  |
| `inotify_init1` | [🐧](https://man7.org/linux/man-pages/man2/inotify_init1.2.html) |  |
| `inotify_rm_watch` | [🐧](https://man7.org/linux/man-pages/man2/inotify_rm_watch.2.html) |  |
| `fanotify_init` | [🐧](https://man7.org/linux/man-pages/man2/fanotify_init.2.html) |  |
| `fanotify_mark` | [🐧](https://man7.org/linux/man-pages/man2/fanotify_mark.2.html) |  |

#### Modify path and FD limits

| Name | OS | Description |
|------|----|-------------|
| `pathconf` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pathconf.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=pathconf&sektion=2), [🐡](https://man.openbsd.org/pathconf.2) |  |
| `fpathconf` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/fpathconf.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=fpathconf&sektion=2), [🐡](https://man.openbsd.org/fpathconf.2) |  |

### Network

#### Berkeley sockets

##### Create

| Name | OS | Description |
|------|----|-------------|
| `socket` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/socket.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=socket&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/socket.2.html), [🐡](https://man.openbsd.org/socket.2) |  |
| `socketpair` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/socketpair.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=socketpair&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/socketpair.2.html), [🐡](https://man.openbsd.org/socketpair.2) |  |

##### Socket lifecycle

| Name | OS | Description |
|------|----|-------------|
| `accept` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/accept.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=accept&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/accept.2.html), [🐡](https://man.openbsd.org/accept.2) |  |
| `accept4` | [😈](https://www.freebsd.org/cgi/man.cgi?query=accept4&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/accept4.2.html), [🐡](https://man.openbsd.org/accept4.2) |  |
| `bind` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/bind.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=bind&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/bind.2.html), [🐡](https://man.openbsd.org/bind.2) |  |
| `connect` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/connect.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=connect&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/connect.2.html), [🐡](https://man.openbsd.org/connect.2) |  |
| `listen` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/listen.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=listen&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/listen.2.html), [🐡](https://man.openbsd.org/listen.2) |  |

##### Socket metadata

| Name | OS | Description |
|------|----|-------------|
| `getpeername` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getpeername.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getpeername&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getpeername.2.html), [🐡](https://man.openbsd.org/getpeername.2) |  |
| `getsockname` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getsockname.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getsockname&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getsockname.2.html), [🐡](https://man.openbsd.org/getsockname.2) |  |
| `getsockopt` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getsockopt.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=getsockopt&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getsockopt.2.html), [🐡](https://man.openbsd.org/getsockopt.2) |  |
| `setsockopt` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setsockopt.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=setsockopt&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/setsockopt.2.html), [🐡](https://man.openbsd.org/setsockopt.2) |  |

##### I/O on a socket

| Name | OS | Description |
|------|----|-------------|
| `recvfrom` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/recvfrom.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=recvfrom&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/recvfrom.2.html), [🐡](https://man.openbsd.org/recvfrom.2) |  |
| `sendto` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sendto.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sendto&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/sendto.2.html), [🐡](https://man.openbsd.org/sendto.2) |  |
| `recvmsg` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/recvmsg.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=recvmsg&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/recvmsg.2.html), [🐡](https://man.openbsd.org/recvmsg.2) |  |
| `sendmsg` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sendmsg.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=sendmsg&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/sendmsg.2.html), [🐡](https://man.openbsd.org/sendmsg.2) |  |
| `recvmmsg` | [🐧](https://man7.org/linux/man-pages/man2/recvmmsg.2.html), [🐡](https://man.openbsd.org/recvmmsg.2) |  |
| `sendmmsg` | [🐧](https://man7.org/linux/man-pages/man2/sendmmsg.2.html), [🐡](https://man.openbsd.org/sendmmsg.2) |  |

#### NFS

| Name | OS | Description |
|------|----|-------------|
| `nfssvc` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/nfssvc.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=nfssvc&sektion=2), [🐡](https://man.openbsd.org/nfssvc.2) |  |
| `nfsservctl` | [🐧](https://man7.org/linux/man-pages/man2/nfsservctl.2.html) |  |

### Polling, multiplexing and asynchronous I/O

#### Polling and multiplexing

| Name | OS | Description |
|------|----|-------------|
| `kevent` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/kevent.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=kevent&sektion=2), [🐡](https://man.openbsd.org/kevent.2) |  |
| `kqueue` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/kqueue.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=kqueue&sektion=2), [🐡](https://man.openbsd.org/kqueue.2) |  |
| `kqueue1` | [🐡](https://man.openbsd.org/kqueue1.2) |  |
| `poll` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/poll.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=poll&sektion=2), [🐡](https://man.openbsd.org/poll.2) |  |
| `ppoll` | [😈](https://www.freebsd.org/cgi/man.cgi?query=ppoll&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/ppoll.2.html), [🐡](https://man.openbsd.org/ppoll.2) |  |
| `pselect` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/pselect.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=pselect&sektion=2), [🐡](https://man.openbsd.org/pselect.2) |  |
| `select` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/select.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=select&sektion=2), [🐡](https://man.openbsd.org/select.2) |  |
| `epoll_create1` | [🐧](https://man7.org/linux/man-pages/man2/epoll_create1.2.html) |  |
| `epoll_ctl` | [🐧](https://man7.org/linux/man-pages/man2/epoll_ctl.2.html) |  |
| `epoll_pwait` | [🐧](https://man7.org/linux/man-pages/man2/epoll_pwait.2.html) |  |
| `epoll_pwait2` | [🐧](https://man7.org/linux/man-pages/man2/epoll_pwait2.2.html) |  |
| `pselect6` | [🐧](https://man7.org/linux/man-pages/man2/pselect6.2.html) |  |

#### POSIX asynchronous I/O

| Name | OS | Description |
|------|----|-------------|
| `aio_read` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/aio_read.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_read&sektion=2) |  |
| `aio_write` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/aio_write.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_write&sektion=2) |  |
| `aio_return` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/aio_return.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_return&sektion=2) |  |
| `aio_suspend` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/aio_suspend.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_suspend&sektion=2) |  |
| `aio_cancel` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/aio_cancel.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_cancel&sektion=2) |  |
| `aio_error` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/aio_error.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_error&sektion=2) |  |
| `aio_waitcomplete` | [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_waitcomplete&sektion=2) |  |
| `aio_fsync` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/aio_fsync.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_fsync&sektion=2) |  |
| `aio_mlock` | [😈](https://www.freebsd.org/cgi/man.cgi?query=aio_mlock&sektion=2) |  |

#### Asynchronous I/O (Linux)

| Name | OS | Description |
|------|----|-------------|
| `io_cancel` | [🐧](https://man7.org/linux/man-pages/man2/io_cancel.2.html) |  |
| `io_destroy` | [🐧](https://man7.org/linux/man-pages/man2/io_destroy.2.html) |  |
| `io_getevents` | [🐧](https://man7.org/linux/man-pages/man2/io_getevents.2.html) |  |
| `io_pgetevents` | [🐧](https://man7.org/linux/man-pages/man2/io_pgetevents.2.html) |  |
| `io_setup` | [🐧](https://man7.org/linux/man-pages/man2/io_setup.2.html) |  |
| `io_submit` | [🐧](https://man7.org/linux/man-pages/man2/io_submit.2.html) |  |
| `io_uring_enter` | [🐧](https://man7.org/linux/man-pages/man2/io_uring_enter.2.html) |  |
| `io_uring_register` | [🐧](https://man7.org/linux/man-pages/man2/io_uring_register.2.html) |  |
| `io_uring_setup` | [🐧](https://man7.org/linux/man-pages/man2/io_uring_setup.2.html) |  |

### Security

#### Sandboxing

| Name | OS | Description |
|------|----|-------------|
| `msyscall` | [🐡](https://man.openbsd.org/msyscall.2) | permit syscalls from a region of pages |
| `pinsyscall` | [🐡](https://man.openbsd.org/pinsyscall.2) | specify the call stub for a specific system call |
| `pledge` | [🐡](https://man.openbsd.org/pledge.2) | force the current process into a restricted-service operating mode |
| `revoke` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/revoke.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=revoke&sektion=2), [🐡](https://man.openbsd.org/revoke.2) | revoke file access |
| `seccomp` | [🐧](https://man7.org/linux/man-pages/man2/seccomp.2.html) | operate on Secure Computing state of the process |
| `unveil` | [🐡](https://man.openbsd.org/unveil.2) | unveil parts of a restricted filesystem view |

#### Jails (FreeBSD)

| Name | OS | Description |
|------|----|-------------|
| `jail` | [😈](https://www.freebsd.org/cgi/man.cgi?query=jail&sektion=2) | sets up a jail and locks the current process in it |
| `jail_attach` | [😈](https://www.freebsd.org/cgi/man.cgi?query=jail_attach&sektion=2) | attaches the current process to an existing jail |
| `jail_get` | [😈](https://www.freebsd.org/cgi/man.cgi?query=jail_get&sektion=2) | retrieves jail parameters |
| `jail_set` | [😈](https://www.freebsd.org/cgi/man.cgi?query=jail_set&sektion=2) | creates a new jail or modifies an existing one |
| `jail_remove` | [😈](https://www.freebsd.org/cgi/man.cgi?query=jail_remove&sektion=2) | removes the jail |

#### Capabilities (Linux)

| Name | OS | Description |
|------|----|-------------|
| `capget` | [🐧](https://man7.org/linux/man-pages/man2/capget.2.html) |  |
| `capset` | [🐧](https://man7.org/linux/man-pages/man2/capset.2.html) |  |

#### Namespaces (Linux)

| Name | OS | Description |
|------|----|-------------|
| `setns` | [🐧](https://man7.org/linux/man-pages/man2/setns.2.html) | reassociate thread with a namespace |
| `unshare` | [🐧](https://man7.org/linux/man-pages/man2/unshare.2.html) | disassociate parts of the process execution context |

#### Landlock (Linux)

| Name | OS | Description |
|------|----|-------------|
| `landlock_create_ruleset` | [🐧](https://man7.org/linux/man-pages/man2/landlock_create_ruleset.2.html) |  |
| `landlock_add_rule` | [🐧](https://man7.org/linux/man-pages/man2/landlock_add_rule.2.html) |  |
| `landlock_restrict_self` | [🐧](https://man7.org/linux/man-pages/man2/landlock_restrict_self.2.html) |  |

#### Entropy and random

| Name | OS | Description |
|------|----|-------------|
| `getentropy` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getentropy.2.html), [🐡](https://man.openbsd.org/getentropy.2) |  |
| `getrandom` | [😈](https://www.freebsd.org/cgi/man.cgi?query=getrandom&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/getrandom.2.html) |  |

#### IPSec keys

| Name | OS | Description |
|------|----|-------------|
| `add_key` | [🐧](https://man7.org/linux/man-pages/man2/add_key.2.html) |  |
| `keyctl` | [🐧](https://man7.org/linux/man-pages/man2/keyctl.2.html) |  |
| `request_key` | [🐧](https://man7.org/linux/man-pages/man2/request_key.2.html) |  |

### Interprocess communication

#### System V semaphores

| Name | OS | Description |
|------|----|-------------|
| `semctl` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/semctl.2.html), [🐧](https://man7.org/linux/man-pages/man2/semctl.2.html) |  |
| `semtimedop` | [🐧](https://man7.org/linux/man-pages/man2/semtimedop.2.html) |  |
| `__semctl` | [😈](https://www.freebsd.org/cgi/man.cgi?query=__semctl&sektion=2), [🐡](https://man.openbsd.org/__semctl.2) |  |
| `semget` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/semget.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=semget&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/semget.2.html), [🐡](https://man.openbsd.org/semget.2) |  |
| `semop` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/semop.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=semop&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/semop.2.html), [🐡](https://man.openbsd.org/semop.2) |  |

#### POSIX message queues

| Name | OS | Description |
|------|----|-------------|
| `msgctl` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/msgctl.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=msgctl&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/msgctl.2.html), [🐡](https://man.openbsd.org/msgctl.2) |  |
| `msgget` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/msgget.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=msgget&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/msgget.2.html), [🐡](https://man.openbsd.org/msgget.2) |  |
| `msgrcv` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/msgrcv.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=msgrcv&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/msgrcv.2.html), [🐡](https://man.openbsd.org/msgrcv.2) |  |
| `msgsnd` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/msgsnd.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=msgsnd&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/msgsnd.2.html), [🐡](https://man.openbsd.org/msgsnd.2) |  |
| `mq_getsetattr` | [🐧](https://man7.org/linux/man-pages/man2/mq_getsetattr.2.html) |  |
| `mq_notify` | [🐧](https://man7.org/linux/man-pages/man2/mq_notify.2.html) |  |
| `mq_open` | [🐧](https://man7.org/linux/man-pages/man2/mq_open.2.html) |  |
| `mq_timedreceive` | [🐧](https://man7.org/linux/man-pages/man2/mq_timedreceive.2.html) |  |
| `mq_timedsend` | [🐧](https://man7.org/linux/man-pages/man2/mq_timedsend.2.html) |  |
| `mq_unlink` | [🐧](https://man7.org/linux/man-pages/man2/mq_unlink.2.html) |  |

#### Shared memory

| Name | OS | Description |
|------|----|-------------|
| `shmat` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/shmat.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=shmat&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/shmat.2.html), [🐡](https://man.openbsd.org/shmat.2) |  |
| `shmctl` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/shmctl.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=shmctl&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/shmctl.2.html), [🐡](https://man.openbsd.org/shmctl.2) |  |
| `shmdt` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/shmdt.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=shmdt&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/shmdt.2.html), [🐡](https://man.openbsd.org/shmdt.2) |  |
| `shmget` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/shmget.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=shmget&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/shmget.2.html), [🐡](https://man.openbsd.org/shmget.2) |  |

### System

#### General

| Name | OS | Description |
|------|----|-------------|
| `reboot` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/reboot.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=reboot&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/reboot.2.html), [🐡](https://man.openbsd.org/reboot.2) |  |
| `shutdown` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/shutdown.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=shutdown&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/shutdown.2.html), [🐡](https://man.openbsd.org/shutdown.2) |  |
| `sysctl` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/sysctl.2.html), [🐡](https://man.openbsd.org/sysctl.2) |  |
| `setdomainname` | [🐧](https://man7.org/linux/man-pages/man2/setdomainname.2.html) |  |
| `sethostname` | [🐧](https://man7.org/linux/man-pages/man2/sethostname.2.html) |  |
| `uname` | [🐧](https://man7.org/linux/man-pages/man2/uname.2.html) |  |
| `sysinfo` | [🐧](https://man7.org/linux/man-pages/man2/sysinfo.2.html) |  |

#### Kernel

##### Kernel module management (Linux)

| Name | OS | Description |
|------|----|-------------|
| `finit_module` | [🐧](https://man7.org/linux/man-pages/man2/finit_module.2.html) |  |
| `init_module` | [🐧](https://man7.org/linux/man-pages/man2/init_module.2.html) |  |
| `delete_module` | [🐧](https://man7.org/linux/man-pages/man2/delete_module.2.html) |  |

##### kexec (Linux)

| Name | OS | Description |
|------|----|-------------|
| `kexec_file_load` | [🐧](https://man7.org/linux/man-pages/man2/kexec_file_load.2.html) |  |
| `kexec_load` | [🐧](https://man7.org/linux/man-pages/man2/kexec_load.2.html) |  |

##### Dynamic kernel linker facility (FreeBSD)

| Name | OS | Description |
|------|----|-------------|
| `kldsym` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldsym&sektion=2) |  |
| `kldload` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldload&sektion=2) |  |
| `kldunload` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldunload&sektion=2) |  |
| `kldunloadf` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldunloadf&sektion=2) |  |
| `kldfind` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldfind&sektion=2) |  |
| `kldnext` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldnext&sektion=2) |  |
| `kldstat` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldstat&sektion=2) |  |
| `kldfirstmod` | [😈](https://www.freebsd.org/cgi/man.cgi?query=kldfirstmod&sektion=2) |  |

#### System log

| Name | OS | Description |
|------|----|-------------|
| `sendsyslog` | [🐡](https://man.openbsd.org/sendsyslog.2) |  |
| `utrace` | [😈](https://www.freebsd.org/cgi/man.cgi?query=utrace&sektion=2), [🐡](https://man.openbsd.org/utrace.2) |  |
| `syslog` | [🐧](https://man7.org/linux/man-pages/man2/syslog.2.html) |  |

#### Swap

| Name | OS | Description |
|------|----|-------------|
| `swapctl` | [🐡](https://man.openbsd.org/swapctl.2) |  |
| `swapoff` | [😈](https://www.freebsd.org/cgi/man.cgi?query=swapoff&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/swapoff.2.html) |  |
| `swapon` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/swapon.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=swapon&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/swapon.2.html) |  |

#### Clock and time functions

| Name | OS | Description |
|------|----|-------------|
| `gettimeofday` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/gettimeofday.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=gettimeofday&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/gettimeofday.2.html), [🐡](https://man.openbsd.org/gettimeofday.2) |  |
| `settimeofday` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/settimeofday.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=settimeofday&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/settimeofday.2.html), [🐡](https://man.openbsd.org/settimeofday.2) |  |
| `clock_gettime` | [😈](https://www.freebsd.org/cgi/man.cgi?query=clock_gettime&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/clock_gettime.2.html), [🐡](https://man.openbsd.org/clock_gettime.2) |  |
| `clock_settime` | [😈](https://www.freebsd.org/cgi/man.cgi?query=clock_settime&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/clock_settime.2.html), [🐡](https://man.openbsd.org/clock_settime.2) |  |
| `clock_getres` | [😈](https://www.freebsd.org/cgi/man.cgi?query=clock_getres&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/clock_getres.2.html), [🐡](https://man.openbsd.org/clock_getres.2) |  |
| `clock_adjtime` | [🐧](https://man7.org/linux/man-pages/man2/clock_adjtime.2.html) |  |
| `adjtimex` | [🐧](https://man7.org/linux/man-pages/man2/adjtimex.2.html) |  |
| `adjfreq` | [🐡](https://man.openbsd.org/adjfreq.2) |  |
| `adjtime` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/adjtime.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=adjtime&sektion=2), [🐡](https://man.openbsd.org/adjtime.2) |  |

#### Quotas and accounting

| Name | OS | Description |
|------|----|-------------|
| `acct` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/acct.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=acct&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/acct.2.html), [🐡](https://man.openbsd.org/acct.2) |  |
| `quotactl` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/quotactl.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=quotactl&sektion=2), [🐧](https://man7.org/linux/man-pages/man2/quotactl.2.html), [🐡](https://man.openbsd.org/quotactl.2) |  |

#### Performance, profiling and and eBPF

| Name | OS | Description |
|------|----|-------------|
| `bpf` | [🐧](https://man7.org/linux/man-pages/man2/bpf.2.html) |  |
| `perf_event_open` | [🐧](https://man7.org/linux/man-pages/man2/perf_event_open.2.html) |  |
| `profil` | [😈](https://www.freebsd.org/cgi/man.cgi?query=profil&sektion=2), [🐡](https://man.openbsd.org/profil.2) |  |

### Meta system calls

| Name | OS | Description |
|------|----|-------------|
| `sysarch` | [😈](https://www.freebsd.org/cgi/man.cgi?query=sysarch&sektion=2), [🐡](https://man.openbsd.org/sysarch.2) |  |
| `__syscall` | [😈](https://www.freebsd.org/cgi/man.cgi?query=__syscall&sektion=2) |  |
| `syscall` | [🍏](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/syscall.2.html), [😈](https://www.freebsd.org/cgi/man.cgi?query=syscall&sektion=2), [🐡](https://man.openbsd.org/syscall.2) |  |
| `arch_specific_syscall` | [🐧](https://man7.org/linux/man-pages/man2/arch_specific_syscall.2.html) |  |
| `restart_syscall` | [🐧](https://man7.org/linux/man-pages/man2/restart_syscall.2.html) |  |
| `rseq` | [🐧](https://man7.org/linux/man-pages/man2/rseq.2.html) |  |
