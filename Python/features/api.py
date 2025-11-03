import yt_dlp


class Features:
    def __init__(self) -> None:
        pass

    def search(self, input: str):
        print("Searching")
        yt_dlp_opts = {}
