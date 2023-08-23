# Introduction to Disambiguations

## Our Meet is Ready
We have finished adding our meet and we think it's ready to go.

> user@machine:~/opl-data/meet-data/spf/2315$ ll -h  
> total 4.0K  
> drwxr-xr-x 1 user user 4.0K Aug 23 19:24 ./  
> drwxr-xr-x 1 user user 4.0K Aug 22 20:56 ../  
> -rw-r--r-- 1 user user   63 Aug 22 20:40 URL  
> -rw-r--r-- 1 user user 1.8K Aug 23 19:24 entries.csv  
> -rw-r--r-- 1 user user   98 Aug 22 20:46 meet.csv  

So we run the checker to make sure it's ready. 

> user@machine:~/opl-data/meet-data/spf/2315$ ../../../tests/check  
>     Finished release [optimized] target(s) in 7.57s  
>     Running `target/release/checker /home/user/opl-data/meet-data/spf/2315`  
> /home/user/opl-data/meet-data/spf/2315/entries.csv  
>   Line 10: Disambiguate Andrew Johnson (https://www.openpowerlifting.org/u/andrewjohnson)  
> Summary: 1 error, 0 warnings for spf/2315  

But we have an error and need to disambiguate this lifter.

## Look at the Lifter's Entry and Webpage
We need to look at the entry for that lifter in our meet.

> Andrew Johnson,Juniors,Wraps,90,88.72,SBD,206.38,120.2,233.6,560.19,M,,1

So we have a Junior 90Kg lifter in SPF.

We look at the page listed in the error (https://www.openpowerlifting.org/u/andrewjohnson). At the time of writing it showed 15 different Andrew Johnsons, none of which appear to be the Andrew Johnson that is in our meet. 

## If our lifter already exists 

If we had identified that our lifter was already on the site as, say Andrew Johnson #8, we would simply change our entries.csv as follows:

> Andrew Johnson #8,Juniors,Wraps,90,88.72,SBD,206.38,120.2,233.6,560.19,M,,1

In this case however we have a new lifter. 

## Our lifter does not yet exist on the site

We need to edit the file that tells the system that there are multiple Andrew Johnsons on the site. That file is opl-data/lifter-data/name-disambiguation.csv.

We open that and look for Andrew Johnson, and we see: 

> Andrew Johnson,15

This tells the system that there are 15 Andrew Johnsons. We need to add a new one, so we change that line to: 

> Andrew Johnson,16

Then we edit our entries.csv file:

> Andrew Johnson #16,Juniors,Wraps,90,88.72,SBD,206.38,120.2,233.6,560.19,M,,1

## We check our meet again

We run the checker again...

> user@machine:~/opl-data/meet-data/spf/2315$ ../../../tests/check  
>     Finished release [optimized] target(s) in 0.59s  
>     Running `target/release/checker /home/user/opl-data/meet-data/spf/2315`  
> Summary: 0 errors, 0 warnings for spf/2315  

And our meet is good. It's worth noting that that this just tests our meet for internal structure and consistency, and does not check against the rest of the data in the project. But for our purposes here, it is good to go.  

