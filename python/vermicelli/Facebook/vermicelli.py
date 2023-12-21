import re

from selenium import webdriver
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.common.exceptions import TimeoutException
import json

# 垃圾文件地址
# /private/var/folders/36/csqnmwj93c7gx0003_19fv1m0000gn

# 打开并读取JSON文件
# with open('./settings.json', 'r') as file:
#     data = json.load(file)

# 打印读取的数据
# print(f"chromedriver：{data['chromedriver']}")
# chromedriver_path = data["chromedriver"]
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


# 等待 元素显示
def web_driver_wait_button_click(utl, timeout=10):
    wait = WebDriverWait(driver, timeout)
    button = wait.until(EC.element_to_be_clickable((By.XPATH, utl)))
    button.click()
    return button


# 打开粉丝页面
driver.execute_script(f"window.location.href = '{utl}/followers';")

# 点击蒙板
web_driver_wait_button_click('//div[@class="__fb-light-mode x1n2onr6 x1vjfegm"]',30)

# 获取用户 ID
def extract_facebook_id_or_username(url):
    # 正则表达式用于匹配两种不同的 Facebook URL
    pattern = r'facebook\.com/(profile\.php\?id=(\d+)|([^/?]+))'
    match = re.search(pattern, url)

    if match:
        # 如果匹配到 'profile.php?id=...' 格式，返回数字 ID
        if match.group(2):
            return match.group(2)
        # 否则返回用户名
        return match.group(3)
    else:
        return None  # 如果不匹配，返回 None


# 设置显式等待
wait = WebDriverWait(driver, 10)
# 底部加载触发元素
trigger_element_xpath = "/html/body/div[1]/div/div[1]/div/div[3]/div/div/div[1]/div[1]/div/div/div[4]/div/div/div/div/div/div/div/div/div[3]/div[contains(@class, 'xh8yej3')]"
# 列表父元素
parent_div = driver.find_element("xpath",
                                 "/html/body/div[1]/div/div[1]/div/div[3]/div/div/div[1]/div[1]/div/div/div[4]/div/div/div/div/div/div/div/div/div[3]")


def get_elem_info():
    return driver.find_elements("xpath",
                                "/html/body/div[1]/div/div[1]/div/div[3]/div/div/div[1]/div[1]/div/div/div[4]/div/div/div/div/div/div/div/div/div[3]/div[contains(@class, 'x6s0dn4')]")  # 替换为列表项的 XPath

processed_ids = set()  # 用来跟踪已处理的元素ID、

# 示例：对象数组（字典的列表）
object_array = []

while True:
    # 滚动到页面底部
    driver.execute_script("window.scrollTo(0, document.body.scrollHeight);")

    try:
        # 滚动到触发元素
        wait.until(EC.presence_of_element_located((By.XPATH, trigger_element_xpath)))

        # 提取页面数据
        elements = get_elem_info()  # 替换为列表项的 XPath
        for element in elements:
            element_href = element.find_element("xpath", "./div[1]/a").get_attribute('href')  # 或者其他能唯一标识元素的属性
            element_id = extract_facebook_id_or_username(element_href)
            user_data = {
                "name": element.find_element("xpath", "./div[2]").text,
                "icon": element.find_element("xpath", "./div[1]/a/img").get_attribute('src'),
                "id": element_id,
                "home_page": element_href,
                "description": element.find_element("xpath", "./div[3]").text,
                "chat": "https://www.facebook.com/messages/t/" + element_id
            }
            if element_id not in processed_ids:
                processed_ids.add(element_id)
                object_array.append(user_data)
                # 将对象数组写入 JSON 文件
                with open("data.json", "w") as file:
                    json.dump(object_array, file, indent=4)

    except TimeoutException:
        # 如果超时，可能已经到达页面底部或加载了所有内容
        break

driver.quit()
