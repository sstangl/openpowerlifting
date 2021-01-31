## Redacting for Privacy

OpenPowerlifting collects data from the public record of sport competitions.

The GDPR operates on the basis of *user consent withdrawal*.
This does not apply to us, because:
1. OpenPowerlifting does not have users.
2. The public record of sport has no consent requirement.

Nevertheless, although not required by law, we want to be sympathetic to people who request privacy.
Usually this happens when they search for their own name and unexpectedly find their sports record.
The proper channel to fix this would be the search engines themselves: but we can help too.

In order to give lifters privacy while preserving the accuracy of the historical record, we can *redact* their names.

### Using the GitLab Edit Tool

Redacted names appear in the file `lifter-data/privacy.csv`.

To add a new name to the redacted list:
 1. Log into a [GitLab](https://gitlab.com) account.
 2. Browse to [lifter-data/privacy.csv](https://gitlab.com/openpowerlifting/opl-data/blob/main/lifter-data/privacy.csv).
 3. Click the `Edit` button, near the top-right corner of the displayed file.
 4. Scroll to where the name should be alphabetically.
 5. Type in a new line containing the lifter's name.
 6. Scroll to the bottom of the page, add a commit message (like "Redact Your Name"), and click `Commit Changes`.
 7. Copy and paste the title of the new merge request into the Description textbox.
 8. Check all three checkboxes next to the phrases:
    - Delete source branch when merge request is accepted
    - Squash commits when merge request is accepted
    - Allow commits from members who can merge to the target branch
 9. Click the `Submit Merge Request` button.

The proposed change is now submitted for automatic testing, manual review, and inclusion.
