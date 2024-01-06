import time

from selenium import webdriver

def main():
    driver = webdriver.Chrome()
    driver.get('http://www.baidu.com')
    time.sleep(5)
    driver.find_element('xpath', '//*[@id="kw"]').send_keys('日期')
    driver.find_element('id', 'su').click()
    time.sleep(5)
    driver.quit()

if __name__ == "__main__":
    main()
