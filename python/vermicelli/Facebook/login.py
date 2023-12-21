import re

from selenium import webdriver
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.support import expected_conditions as EC

chromedriver_path = '/Users/Fox/Code/Rust/Arachnoid/python/chromedriver'
utl = "https://www.facebook.com/andreijames.diaz"

# 指定 chromedriver 的路径
service = Service(chromedriver_path)
driver = webdriver.Chrome(service=service)
driver.get(utl)

# 找到用户名和密码输入框
username_input = driver.find_element("xpath",
                                     '//*[@id="login_popup_cta_form"]/div/div[3]/div/label/div/div/input')  # 替换为用户名输入框的实际 id 或其他定位器
password_input = driver.find_element("xpath",
                                     '//*[@id="login_popup_cta_form"]/div/div[4]/div/label/div/div[1]/input')  # 替换为密码输入框的实际 id 或其他定位器
# 输入用户名和密码
username_input.send_keys("glinfen@gmail.com")  # 替换为你的用户名

password_input.send_keys("951299034.glf")  # 替换为你的密码

driver.find_element("xpath", '//*[@id="login_popup_cta_form"]/div/div[5]/div/div').click()

