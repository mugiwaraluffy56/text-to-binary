# Security

## Reporting a vulnerability

If you find a security issue, please do not open a public GitHub issue.

Email the maintainer directly or use GitHub's private vulnerability reporting if available on this repo.

Include:
- Description of the issue
- Steps to reproduce
- Potential impact

You should get a response within 72 hours. If you don't, follow up.

## Scope

binrs is a local CLI tool. It reads files from disk and writes to stdout or a specified output path. It does not make network requests, spawn subprocesses, or use eval-style dynamic execution.

Known non-issues:
- Processing attacker-controlled input files is expected use
- Output goes where the user specifies via `-o`
