import telebot
import requests

omdbapi = "97773e90"
bot_id = '5768454504:AAFjiTEsNfJ80NPggzlukAIMEKlhXnqnR68'
bot = telebot.TeleBot(
    bot_id, parse_mode=None)


def movie_info(movie_name):
    movie_name = movie_name.replace(' ', '+')
    url = "https://www.omdbapi.com/?apikey={}&t={}".format(
        omdbapi, movie_name)
    data = requests.get(url).json()
    if (data['Response'] == 'False'):
        return False

    return movie(data)


def movie(data):
    response = "Title: {}\nYear: {}\nRating: {}\nPoster: ".format(
        data.get("Title"), data.get("Year"), data.get("imdbRating"), data.get("Poster"))
    return response


@bot.message_handler(commands=['start', 'hello'])
def send_welcome(message):
    global botRunning
    botRunning = True
    bot.reply_to(
        message, "Hello there! I am a bot that will show movie information for you and export it in a CSV file.\n\n")


@bot.message_handler(commands=['stop', 'bye'])
def send_bye(message):
    global botRunning
    botRunning = False
    bot.reply_to(message, "Bye!\nHave a good time")


@bot.message_handler(func=lambda message: botRunning, commands=['help'])
def send_help(message):
    global botRunning
    botRunning = True
    bot.reply_to(message, "1.0 You can use \"/movie MOVIE_NAME\" command to get the details of a particular movie. For eg: \"/movie The Shawshank Redemption\"\n\n2.0. You can use \"/export\" command to export all the movie data in CSV format.\n\n3.0. You can use \"/stop\" or the command \"/bye\" to stop the bot.")


@bot.message_handler(func=lambda message: botRunning, commands=['movie'])
def getMovie(message):
    message.text = message.text.lower()
    movieName = message.text.removeprefix('/movie ')
    bot.reply_to(message, 'Getting movie info...')
    global botRunning
    botRunning = True
    movieName = movieName.strip()
    movieData = movie_info(movieName)
    if (movie_info(movieName) == False):
        bot.reply_to(message, 'Movie not found')
        return
    bot.reply_to(message, movieData)
    write_CSV(movieData)


def write_CSV(movieData):
    titles = ["Title: ", "Year: ", "IMDb Rating: "]
    for title in titles:
        movieData = movieData.replace(title, "")
    movieData = movieData.replace("\n", ",")
    file = open("movie_data.csv", "a+")
    file.write(movieData + '\n')
    file.close()
    return True


@bot.message_handler(func=lambda message: botRunning, command=['export'])
def getList(message):
    bot.reply_to(message, "Generating CSV file...")
    with open("movie_data.csv", "rb") as file:
        bot.send_document(message.chat.id, document=file,
                          visible_file_name="stats.csv")


@bot.message_handler(func=lambda message: botRunning)
def default(message):
    bot.reply_to(message, "I didn't understand "+'\N{confused face}')


bot.infinity_polling()
