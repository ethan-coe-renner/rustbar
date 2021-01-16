# rustbar
a simple status bar written in rust for swaybar

cron jobs are used to put some information (updates, todos, and unread rss notifications) into files in .local/share.
This bar reads those to display the info

rustbar was designed for use in swaybar, as such it outputs in json format to get colors, etc,
however it should also work in i3bar, but I haven't tried it there
