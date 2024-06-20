import os
while True:
    os.system("clear")
    os.system("figlet -f slant -c Booter")
    os.system("./lexce nt show")
    choice = input("Enter your choice: ")
    if choice == '1':
        os.system("./kern-arm64")
    elif choice == '2':
        os.system("reboot")
    elif choice == '3':
        os.system("shutdown -P now")
    else:
        print("Invalid choice")
