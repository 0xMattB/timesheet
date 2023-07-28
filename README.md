This is a practice program, which I am able to use for a specific purpose at work.
This has been uploaded to show my progress in learning Rust.

The software we use to log into and out of jobs can print records. At the beginning of each week, we print the previous weeks' data and use the times to fill in our timesheets, where time for each job is tracked. Interpreting the records and and transferring that information to the timesheet file is usually done much faster, so it was better to write a program that can perform this task.

This program takes an input file as an argument, and outputs the converted information to a file, as well as printing the output to the screen. Having the information in an output file is necessary to preserve the tabs, which allows this information to be easily copy/pasted into the timesheet spreadsheet.

It was a fun challenge to figure out how to best store, process, and print the data.
There are many improvements and additions that could be made, but this works sufficiently for my needs, especially considering it is a practice program.