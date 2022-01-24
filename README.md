# Personio tool

## How to use

- Create a folder with the binary and a config.json with this (example):

{<br>
&emsp;"credentials": {<br>
&emsp;&emsp;"email": "your_email@personio",<br>
&emsp;&emsp;"password": "your_password"<br>
&emsp;},<br>
&emsp;"dates": {<br>
&emsp;&emsp;"startDay": "2022-01-28",<br>
&emsp;&emsp;"endDay": "2022-01-28"<br>
&emsp;},<br>
&emsp;"times": {<br>
&emsp;&emsp;"breakStartHour": "14:00:00.0",<br>
&emsp;&emsp;"breakDuration": 30,<br>
&emsp;&emsp;"breakRandomMinutesDelta": 30,<br>
&emsp;&emsp;"workStartHour": "08:00:00.0",<br>
&emsp;&emsp;"workDuration": 8,<br>
&emsp;&emsp;"workRandomMinutesDelta": 30<br>
&emsp;},<br>
&emsp;"params": {<br>
&emsp;&emsp;"delayBetweenRequestsMs": 1000,<br>
&emsp;&emsp;"timeoutRequestSeconds": 20<br>
&emsp;}<br>
}

- Open the terminal, execute the file
    - Windows: ``.\personio_tool``
    - Mac: ``./personio_tool``