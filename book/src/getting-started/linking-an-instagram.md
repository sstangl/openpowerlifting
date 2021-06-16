## Linking an Instagram

OpenPowerlifting can optionally associate lifters with their Instagram profiles.

The list of `Name` to `Instagram` associations is tracked by the file [lifter-data/social-instagram.csv](https://gitlab.com/openpowerlifting/opl-data/blob/main/lifter-data/social-instagram.csv).

An example of the table format is below:

| Name              | Instagram           |
|-------------------|---------------------|
| Andrey Malanichev | andrey\_malanichev  |
| Dan Green         | dangreenpowerlifter |
| Ed Coan           | eddycoan            |

Note that the table entries are in alphabetical order by Name.

### Using the GitLab Edit Tool

Because the associations are made in a single file, it is particularly convenient to add an Instagram using GitLab's online editor:
 1. Log into a [GitLab](https://gitlab.com) account.
 2. Browse to [lifter-data/social-instagram.csv](https://gitlab.com/openpowerlifting/opl-data/blob/main/lifter-data/social-instagram.csv).
 3. Click the `Edit` button, near the top-right corner of the displayed file.
 4. Scroll to where the name should be alphabetically.
 5. Type in a new line, in the format `Name,Instagram`, with no spacing before or after the comma, like the lines around it.
 6. Scroll to the bottom of the page, add a commit message (like "Add IG for Your Name"), and click `Commit Changes`.
 7. Copy and paste the title of the new merge request into the Description textbox.
 8. Check all three checkboxes next to the phrases: 
	- Delete source branch when merge request is accepted
	- Squash commits when merge request is accepted
	- Allow commits from members who can merge to the target branch
 9. Click the `Submit Merge Request` button.

The proposed change is now submitted for automatic testing, manual review, and inclusion.

Please note that if your GitLab account is new, it will report that tests have failed.
That is a safety mechanism, and we will still be able to include your changes.
