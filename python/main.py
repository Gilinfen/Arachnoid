# main.py

import sys
from license_verification import verify_license

def main():
    # if len(sys.argv) < 3:
    #     print("Usage: python main.py [app1|app2] [license_key]")
    #     sys.exit(1)

    # license_key = sys.argv[2]
    # if not verify_license(license_key):
    #     print("Invalid or expired license key. Access denied.")
    #     sys.exit(1)

    if sys.argv[1] == 'app1':
        print("Running app1")
    elif sys.argv[1] == 'app2':
        print("Running app2")
    else:
        print("Invalid argument. Please use 'app1' or 'app2'.")

if __name__ == "__main__":
    main()
