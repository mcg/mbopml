import requests
import argparse
import xml.etree.ElementTree as ET

def download_following_list(api_key, username, format):
    url = f"https://micro.blog/users/following/{username}"
    headers = {"Authorization": f"Bearer {api_key}"}
    response = requests.get(url, headers=headers)
    
    if response.status_code == 200:
        following_list = response.json()
        opml = generate_opml(following_list, format)
        print(f"{opml}")
    else:
        print(f"Failed to download following list. Status code: {response.status_code}")

def generate_opml(following_list, format):
    root = ET.Element("opml")
    head = ET.SubElement(root, "head")
    body = ET.SubElement(root, "body")
    
    for user in following_list:
        outline = ET.SubElement(body, "outline")
        outline.set("text", user["name"])  # Change outline text to be the feed's title
        outline.set("type", "rss")
        outline.set("title", user["name"])  # Add a new field called "title" and set its value to the feed's title
        if format == "xml":
            outline.set("xmlUrl", f"https://{user['username']}.micro.blog/feed.xml")
        elif format == "json":
            outline.set("xmlUrl", f"https://{user['username']}.micro.blog/feed.json")
    
    tree = ET.ElementTree(root)
    opml_str = ET.tostring(root, encoding="utf-8", method="xml").decode()
    return opml_str

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--api-key", help="Micro.blog API key")
    parser.add_argument("--username", help="Username of the user to download following list")
    parser.add_argument("--format", help="Format of the feed (xml or json)")
    args = parser.parse_args()

    if args.api_key and args.username and args.format:
        download_following_list(args.api_key, args.username, args.format)
    else:
        print("Please provide the Micro.blog API key, username, and format using the --api-key, --username, and --format arguments.")
