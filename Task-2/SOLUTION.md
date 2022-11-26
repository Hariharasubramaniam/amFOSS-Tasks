<center> <b> <u> DESCRIPTION OF THE TERMINAL COMMANDS </u> </center> </b>

Step-1: To start this task we have to clone the GitHub repository to our device. This can be achieved by the command:
 git clone https://github.com/gauthamk02/TerminalHunt.git
 
Step-2: Now we have to create a new directory named solution in the same repository. This can be done using the terminal command:
 `mkdir solution `
 (mkdir - make directory)

Step-3: Now navigate to the solution folder using the ` cd ` command:
 Like ` cd "[Path]" `
 
Step-4: Create a file named "part1.txt" using the command:
 ` touch part1.txt `
 
Step-5: Product of smallest perfect number(=6) and 44 is 264. Element with atomic mass=264 is Bohrium and it's atomic number is 107.

Step-6: Write 107 into part1.txt using the command:
`echo 107 > part1.txt`
 
Step-7: For the second part of the password, find the HCF of 336 and 702(=6)

Step-8: Then, navigate to the 6th folder in the TerminalHunt directory using the ` cd  ` command:

Step-9: Fourth digit of pi is 1(3.1416..). So, copy the 1.txt file in the 6th folder to the solution directory using the ` cp  ` command:
 ` cp "[Path1]" "[Path2]" `

Step-10: Now rename the file as "part2.txt" using the `mv ` command:
 `mv "[Name]" "[NewName]" `
 
Step-11: To get the commit log of the repository; first,navigate to the TerminalHunt folder using the ` cd  ` command and then type the command:
 `git log`
 The clue for the next part of the password will be visible.
 
Step-12: Copy the file mentioned in the clue using the ` cp  ` command. Navigate to the solution folder using the ` cd  ` command and then rename it using the ` mv `command.

Step-13: To commit the work to the main branch, first the following command was used:
 `git branch -M main`
 and then to commit the work, the following command was used:
 `git commit -m "Initial commit" `
 
Step-14: The 4th part of the password was stored in a different branch. So, the following command helps us to see all the branches of the repository:
 `git branch -a `

Step-15: To switch to a new branch as per the given clue, the following command was used:
 ` git checkout <branch_name>` .(Here <branch_name> was the name of the largest continent)

Step-16: Finally for finding the file:
 `find . -name  <file_name>.txt ` command was used. Here, <file_name> was the name of the Greek Goddess of war.

Step-17: As the solution directory didn't have the folder, this branch should be merged with the main to get the folder on the main branch. For this, the following command was used: 
 `git checkout main `
 And, merging was achieved with the help of the command:
 `git merge <branch_name> ` where <branch_name> was the name of the branch with the file(Here, name of the largest continent)

Step-18: To copy the file <file-name>.txt(Name of the Greek goddess of war) into the solution directory, `cp `command was used:
 `cp [Path1] [Path2] ` 

Step-19: Then, rename the copied file to part4.txt using the command:
 `mv "[path/<file-name>.txt]" "[path/part4.txt]" `

Step-20: To combine all the 4 parts of the password, the following command was used:
 `cat part1.txt part2.txt part3.txt part4.txt > password.txt `

Step-21: To delete all the other files except the the password.txt file, this command was used:
 `rm part1.txt part2.txt part3.txt part4.txt `
 
Step-22: After these steps, the solution folder had only the "password.txt" file which contained the password to open the protected PDF.

<center> <b> <u> SCREENSHOT OF THE PDF </u></center>

 <img src="./Success.png" />.


 
 
