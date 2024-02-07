# Script that does the job of ssf_prober, ssf_import and assign_date all in one.

import ssf_probe
import ssf_import
import assign_date

meets = ssf_probe.main()
folders = []
for meet in meets:
    try:
        folder = ssf_import.main(meet, False)
        if folder:
            folders.append(folder)
    except Exception as e:
        print(e)
assign_date.main(folders)
