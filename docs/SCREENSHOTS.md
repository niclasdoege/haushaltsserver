# Screenshots and visual assets

## What's in the repository

- `docs/assets/hero.svg`: a bespoke, scalable README banner.
- `docs/assets/dashboard-preview.svg`: a **conceptual** dashboard illustration, intentionally using fictitious residents and data. It is **not** a screenshot of the current application.
- `static/Haushaltsserver.png` and `static/Haushaltsserver_old.png`: existing PNG assets retained as-is; their provenance and suitability as a current UI screenshot have not been independently verified.

The illustrations are deliberately separate from live screenshots, so visitors are not misled about the present state of the WebAssembly UI.

## Capturing a real screenshot

1. Run the service locally after reviewing the [startup and security guidance](../README.md#getting-started). Use a database and JSON fixtures with **invented resident names, dates and email addresses**.
2. If login is required, authenticate with a demo account. Check the page for names, addresses, browser extensions, tokens and confirmation URLs before capturing.
3. Pick an uncluttered, representative view at a desktop width around 1440 px, with a second mobile shot if the current UI renders well.
4. Capture with your browser's screenshot feature or, optionally, the Playwright CLI:

   ```bash
   npx playwright install chromium
   npx playwright screenshot --browser chromium --viewport-size "1440,900" \
     --wait-for-timeout 1500 http://127.0.0.1:8020 docs/assets/dashboard-real.png
   ```

   Replace the example URL with the address actually configured for Rocket. This command is **not** an automated test and has not been run against a live instance.

5. Verify that no personal data is visible. Remove or blur anything sensitive **before** committing. Prefer recapturing with synthetic data over merely masking live data.
6. When the real capture is verified, add a clearly labeled screenshot below the illustrative concept in the README, or replace the concept with the actual capture and update its caption.

Never commit real account emails, household schedules, LAN device addresses, credentials or confirmation-link tokens in screenshots.
