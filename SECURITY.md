# Security & privacy notes

This is a personal, older household automation project. **Do not deploy this repository directly on the public internet without a security review.**

## Checklist before running it with real data

- Move **SMTP credentials, recipient addresses, Rocket's secret key, service URLs and LAN device identifiers** out of source control into appropriately protected configuration. Rotate any credential that was ever committed; deleting it from the newest commit does not remove Git history.
- Review the response CORS fairing in `src/main.rs` and authorization on each changing endpoint. Do not assume that login alone protects every action or confirmation link.
- Review the treatment and entropy of email action tokens, account registration, session lifetimes and logout behavior.
- Use HTTPS, least-privilege execution, scoped network access and local firewall rules. Avoid running `arp-scan` through unrestricted `sudo`.
- Examine `private/pending.json`, `static/example.json` and `static/zustande.json` for historical or personal data before making screenshots, publishing a demo or forking the repository.
- Use synthetic fixtures for documentation. Keep runtime databases, mail settings and backups out of Git.
- Update dependencies and validate with automated tests before connecting the service to a production household.

**Important:** A `.gitignore` rule does not remove a file already tracked by Git, and rewriting a file does not remove its contents from the repository's history. If a previously committed credential was valid, rotate/revoke it and plan a coordinated history cleanup.

Please report security concerns privately to the repository owner rather than posting live secrets or exploitable details in a public issue.
