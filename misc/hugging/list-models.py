from huggingface_hub import list_models

myModels = []

for model in list_models(language="en", task="text-generation"):
  if not model.private and model.disabled == None and model.likes > 1000:
     myModels.append(( model.created_at, model.id ))

myModels.sort(key=lambda x:x[0])

for model in myModels:
   print(model[1])
