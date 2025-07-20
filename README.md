## apifs

apifs (a program intended for seniles) is a tool for setting reminders and writing notes.

![apifs logo](./apifs.png)

### Set reminders and notes

Set reminders from the command line. Reminders can be set to go off once, daily (you can customize which days), or with specific time intervals.

For example you can set a reminder to happen on 2025-08-14 9:31:42 with the description "Do your homework" using the command `apifs notify once -w 2025-08-14 9:31:42 --desc Do your homework`.

If you want to be notified at noon on weekdays, you could use `apifs notify daily -w 12:00:00 -d 1-5`, or if you only want to be notified on Mondays and Wednesdays modify the `1-5` from the previous commands to `13`.

For reminders to occure after 2025-11-11 within time intervals of one and a half hours try `apifs notify interval -w 2025-11-11 -i 1h30m`.

To create a note, hit `apifs note -n <note name> --desc <some_description>`.

### How to actually get reminders 

For reminders to actually happen, an apifs server needs to be running. For this `apifs start` will do the trick, and if you want to stop it just ado `apifs stop`

Note: Only one apifs instance can be run by the same user.

### For more info check out src/help.txt
