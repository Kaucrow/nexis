from colorama import Fore, Style, init

init(autoreset=True)

def red(text):
    return f"{Fore.RED}{text}{Style.RESET_ALL}"