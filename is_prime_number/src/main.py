from os import system


def get_number():
    try:
        input_number = int(input("Verify if a number is prime or not :\n>>> "))
        return input_number
    except ValueError:
        system('cls | color c')
        print("Please ener a positive number")
    


def check_number(number):
    checking_history = []
    for i in range(2, number):
        if input_number%i == 0:
            checking_history.append(False)
        else:
            checking_history.append(True)
    return checking_history


def is_prime(history):
    if False in history:
        print(f"Number : {input_number} is not a prime number.")
    else:
        print(f"Number {input_number} is a prime number.")


while True:
    system('cls')
    input_number = get_number()
    number_history = check_number(input_number)
    is_prime(number_history)
    input("Press ENTER to continue...")
