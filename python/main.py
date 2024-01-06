import sys

def main():
    # 检查参数数量
    if len(sys.argv) <= 1:
        print("No arguments provided. Running default operation.")
        # 在这里可以放置默认操作
        return
    
    print(sys.argv)
    if sys.argv[1] == 'app1':
        print("Running app1")
    elif sys.argv[1] == 'app2':
        print("Running app2")

    else:
        print("Invalid argument. Please use 'app1' or 'app2'.")

if __name__ == "__main__":
    main()
