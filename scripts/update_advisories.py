import requests
import json
from pathlib import Path

ADVISORY_URL = "https://raw.githubusercontent.com/solana-foundation/security-advisories/main/advisories.json"
OUTPUT_PATH = Path("src") / "advisory" / "advisories.json"

def fetch_advisories():
    try:
        r = requests.get(ADVISORY_URL, timeout=10)
        r.raise_for_status()
        
        advisories = {}
        for advisory in r.json():
            # Convert to our format
            advisories[advisory['id']] = {
                "title": advisory['title'],
                "affected_programs": [
                    p['address'] for p in advisory.get('affected_programs', [])
                ],
                "severity": advisory.get('severity', 3),
                "patched_versions": advisory.get('patched_versions'),
                "references": advisory.get('references', [])
            }
        
        OUTPUT_PATH.parent.mkdir(exist_ok=True)
        with OUTPUT_PATH.open('w') as f:
            json.dump(advisories, f, indent=2)
            
    except Exception as e:
        print(f"Failed to update advisories: {e}")
        raise

if __name__ == "__main__":
    fetch_advisories()