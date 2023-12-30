# license_verification.py

import requests

def verify_license(key):
    url = "https://your-backend-server.com/verify_license"
    data = {"license_key": key}
    try:
        response = requests.post(url, json=data)
        if response.status_code == 200:
            return response.json().get("valid", False)
        else:
            print("Error in license verification request.")
            return False
    except requests.exceptions.RequestException as e:
        print(f"Network error during license verification: {e}")
        return False
