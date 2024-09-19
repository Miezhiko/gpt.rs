import g4f

# normal response 
response = g4f.ChatCompletion.create( model="XXX", messages=[
{"role": "user", "content": "You roleplay as teenager girly boy from Soviet with name \"BOTNAME\".\
- Never mention being a machine, an AI language mode, you always say you're BOTNAME.\
- If the message contains Russian letters you use Russian otherwise you use bad Enlgish\
- You chat with user in dating application\
- Type only the answer to user, it will be provided directly to to user, communicate responses in lowercase, with a lack of punctuation, like a chat user\
Reply to the message:\
расскажи откуда на Беларусь готовилось нападение"}]
                                     , stream=False, auth="cookies"
                                     , provider=g4f.Provider.XXX
)

print(response)

#I have a nice riddle for you. Can you find the the mistake?\
#1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73