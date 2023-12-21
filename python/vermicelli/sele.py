import time

from selenium import webdriver
from selenium.webdriver.chrome.service import Service
chromedriver_path = '/Users/Fox/Code/Rust/Arachnoid/python/chromedriver'

# 指定 chromedriver 的路径
service = Service(chromedriver_path)
# driver = webdriver.Chrome()
driver = webdriver.Chrome(service=service)
driver.get('http://www.baidu.com')
# driver.implicitly_wait(10)
time.sleep(5)
driver.find_element('xpath', '//*[@id="kw"]').send_keys('日期')
driver.find_element('id', 'su').click()
time.sleep(5)
driver.quit()
