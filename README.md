# quid

Network Analysis Tool for QuickBooks Servers.

## Rationale

Administrating a QuickBooks environment is a task that can be rife with bugs, network instability, and sudden crashes. [Frequent H202 errors](https://quickbooks.intuit.com/learn-support/en-us/do-more-with-quickbooks/error-code-h202-problem-with-multi-user-hosting-setup/00/1295736) can cause headaches with little to no help online, especially for a highly custom server setup. Thus the need for a tool that fits the sysadmin or business-person and gives enough detail to make meaningful progress towards a stable experience.

Rust was chosen for its exceptional efficiency as a low-overhead programming language. The lack of garbage collection moves the headache of balancing performance at development time, rather than at execution. A language like Python would potentially get in the way and potentially [increase the very issues the diagnostic is attempting to solve](https://www.instagram.com/reel/DDNcWYISEn2/?hl=en). The language has moved to a point of maturity suitable for a skilled end-user and easily compiles for the layman.

## Download

```powershell
git clone https://github.com/spiritualhost/quid.git
```

## Dependencies

To build from source, download [rustup](https://rust-lang.org/tools/install/).

## Build

```powershell
cd quid
cargo build --release
```

Output will be located in the `target/release` directory.

## Usage

quid (**Qu**ickBooks **Id**entifier) can run in the command line. The default action of the program is to perform a survey of network sockets (TCP/UDP) for all current connections to the QuickBooks server ports. This allows us to get a better idea of the current network status of the server, by connection, by multiple granular diagnostics.

Run the following to get the options for quid:

```powershell
$ .\quid.exe --help

Perform helpful network survey for QuickBooks server.

Usage: quid.exe [OPTIONS] --survey <SURVEY>

Options:
  -s, --survey <SURVEY>        
  -t, --t-between <T_BETWEEN>  [default: 5]
  -h, --help                   Print help
  -V, --version                Print version
```

The program needs a survey time to run in seconds, which will be the period for which the scan takes place, with an option to define a time between each scan (which will otherwise happen every 5 seconds for the survey period and may include a marginal amount of processing time).

At the bare minimum:

```powershell
.\target\release\quid.exe -s|--survey {survey_time_seconds}
```

Each iteration will show the state of the server's connections from various remote IPs:

```powershell
+----------------+-------------+------+-------------+-------------+-----+
| destination_ip | source_ip   | port | remote_port | state       | pid |
+----------------+-------------+------+-------------+-------------+-----+
| 192.168.10.200 | 172.25.0.68 | 445  | 49426       | ESTABLISHED | 4   |
+----------------+-------------+------+-------------+-------------+-----+
```

## Configuration

Configuration of specific focus ports can be done in the `src/config.toml`. To find the correct ports for your QuickBooks server, open QBServerUtilityMgr -- the tab Port Manager will list all of them.

This is also where additional diagnostic fields are expressed as booleans to expand the capabilities of analyzing each connection.

### Additional Diagnostics

Use the `config.toml` located in the same directory as the `main.rs` to add or remove certain diagnostics. Adding diagnostics will naturally add time to each survey. Current options are:

- Hostname lookup (DNS)

## Future Improvements

- Integration of UDP in a far more robust way
- Exclusion of columns in final table with None values
- Improve caching to reduce double-checks
