from datasets import load_dataset, Audio, Dataset
import os
from typing import List, Dict
from collections import deque


def makeLabel(
        labels :  List[str]
    ): 

    label2id, id2label = dict(), dict()

    for id, label in enumerate(labels):
        label2id[label] = id
        id2label[str(id)] = label
        
    return label2id, id2label


def createDataset(
        path: str
) -> Audio:
        dirs : List[str] = [items for items in os.walk(path)]
        dataset : Dict[str, str] = {}  #for now trivial with no implementation of nested playlists
        tracks = []
        #TODO : implement a ghraph with nested playlists later.
        for path, subdirectory, files in dirs:
                if subdirectory:
                        dir_stack = deque(subdirectory)

                for file in files:
                        tracks.append(file)                  
                        dataset[path.replace("\\","/") + "/" + file] = dir_stack[0]

                if not subdirectory:
                        dir_stack.popleft()

        ### update labels here to id
        label2id, id2label = makeLabel(dataset.values())

        for key, value in dataset.items():
                dataset[key] = label2id[value]
        
        # music = Dataset.from_dict({"audio" : dataset.keys(),
        #                 "label" : dataset.values()})\
        #                 .train_test_split(test_size=0.3)\
        #                 .cast_column("audio", Audio())
        
        return dataset, label2id, id2label





