# build-report-ingestor [![Check Rust code](https://github.com/msfjarvis/build-report-ingestor/actions/workflows/test.yml/badge.svg)](https://github.com/msfjarvis/build-report-ingestor/actions/workflows/test.yml)

An attempt at building a Gradle Enterprise-like dashboard for Kotlin Build Reports.

### TODO

- [x] Expose an HTTP endpoint for accepting build reports
- [ ] Persist reports to disk
- [ ] Expose a server-driven web dashboard that shows all reports
    - [ ] Detailed view for individual reports
    - [ ] Track trends for individual metrics across multiple reports
