# Deleting your repo

There are a few reasons why you need to do this but, most commonly, it's because you have committed to your main branch and have therefore diverged from upstream.

Here’s the easiest way to fix it.  Delete everything. 
It sounds dramatic but if you think that you are repeatedly taking a snapshot of the project to work on, submitting it to be merged, and pulling an update, it’s not much different.  You are just ditching a slightly corrupted copy and getting a fresh new one. 
Here’s how to do it.  Go to your own page on Gitlab, rather than the project page. Check the URL on the screenshot below and substitute your own user name.  You can see that my main branch is 28 commits behind the upstream repository.  If you have committed to main, yours may also say X commits ahead.  This is your indicator that you have committed to main. 
Go to settings bottom left and select "general".

Scroll to the bottom and choose "Advanced"

Scroll right to the bottom again and click delete. You will get all manner of dire warnings but confirm your choice and your repo will be deleted.

Often you will be told that your repo is scheduled to be deleted in one month.  However, if you got back to Settings > General > Advanced and scroll right to the bottom, you will see that you now have the option to delete immediately.

You can now fork the project again and get a fresh clean copy.

