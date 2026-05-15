# quid

Network Analysis Tool for QuickBooks Servers.

## Rationale

Administrating a QuickBooks environment is a task that can be rife with bugs, network instability, and sudden crashes. [Frequent H202 errors](https://quickbooks.intuit.com/learn-support/en-us/do-more-with-quickbooks/error-code-h202-problem-with-multi-user-hosting-setup/00/1295736) can cause headaches with little to no help online, especially for a highly custom server setup. Thus the need for a tool that fits the sysadmin or business-person and gives enough detail to make meaningful progress towards a stable experience.

Rust was chosen for its exceptional efficiency as a low-overhead programming language. The lack of garbage collection moves the headache of balancing performance at development time, rather than at execution. A language like Python would potentially get in the way and potentially [increase the very issues the diagnostic is attempting to solve](https://www.instagram.com/reel/DDNcWYISEn2/?hl=en). The language has moved to a point of maturity suitable for a skilled end-user and easily compiles for the layman.

## Usage

quid (**Qu**ickBooks **Id**entifier) can run either in the command line or as an executable, whichever is most convenient. The default action of the program is to perform a survey of network sockets (TCP/UDP) for all current connections to the QuickBooks server ports. This allows us to get a better idea of the current network status of the server, by connection, by multiple granular diagnostics.

