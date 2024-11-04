
import numpy as np

from pathlib import Path


import os
import torch
from typing import List

from make_spec import SpecLoader
from make_dataset import createDataset
from extract_audio_features import featExtractor, trainModel
from make_dataset import makeLabel

import time



if __name__ == "__main__":
    path : str = "C:/Users/6090249/Desktop/Learning/RustyMusic/Playlist"
    dataset, label2id, id2label = createDataset(path)


    audio_in = dataset.map(featExtractor,
                           batched=True)
    label2id_train, label2id_test = makeLabel(audio_in["train"]["label"]), makeLabel(audio_in["test"]["label"])
    audio_in["train"]["label"] = label2id_train
    audio_in["test"]["label"] = label2id_test
    audio_in =  audio_in.remove_columns("audio")

    # hugging face loader takes 340 secs


    # for i in range(yes["train"].num_rows):
    #     print(yes["train"][i]["audio"]["array"])
        
    # print((end-start))

    
    with torch.no_grad():
        trainer = trainModel(
            audio_in=audio_in,
            label2id=label2id,
            id2label=id2label
        ).train()  ## no label 2 id function hence the error
    
    breakpoint()




    # below in case transformers dont work!
    # s = SpecLoader(path)
    # y, sr, S_db = s.loadAudio()
    # s.plotSpec(S_db, sr)