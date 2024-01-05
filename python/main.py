# main.py

import sys
# from license_verification import verify_license
from utils.handleProduct import copy_files_to_new_folders
from utils.create_xls import create_xls_with_dropdown

def main():
    # 检查参数数量
    if len(sys.argv) <= 1:
        print("No arguments provided. Running default operation.")
        # 在这里可以放置默认操作
        return
    
    #     print("Usage: python main.py [app1|app2] [license_key]")
    #     sys.exit(1)

    # license_key = sys.argv[2]
    # if not verify_license(license_key):
    #     print("Invalid or expired license key. Access denied.")
    #     sys.exit(1)

    print(sys.argv)
    if sys.argv[1] == 'app1':
        print("Running app1")
    elif sys.argv[1] == 'app2':
        print("Running app2")
    elif sys.argv[1] == '-pm':
        if len(sys.argv) < 4:
            print("[base_path] [target_path] [remove_strings]")
            sys.exit(1)
        copy_files_to_new_folders(sys.argv[2],sys.argv[3],sys.argv[4])
    elif sys.argv[1] == '-pmxls':
        if len(sys.argv) < 3:
            print("[base_path] [target_path]")
            sys.exit(1)
        create_xls_with_dropdown(sys.argv[2],sys.argv[3])
    else:
        print("Invalid argument. Please use 'app1' or 'app2'.")

if __name__ == "__main__":
    main()
