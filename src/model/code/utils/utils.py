import csv
import os
import xml.dom.minidom as minidom

import shutil
import yaml
import json
import shutil
import time
from munch import Munch

import logging


def get_wikitext(filepath, datapath):

    list_dir = os.listdir(filepath)

    with open(datapath+"wikitext.csv", "w") as csvfile:
        writer = csv.writer(csvfile)
        writer.writerow(["index", "text_name", "text"])
        for file_name in list_dir:
            file_path = datapath+file_name
            tree = minidom.parse(file_path)
            root = tree.documentElement
            writer.writerow([root.getAttribute("id"),
                             root.getAttribute("name"),
                             root.getElementsByTagName("text")[0].childNodes[0].data])


class Logger():
    def __init__(self, filepath='log/', **kwargs):
        fmt = kwargs.pop('format', "%(asctime)s - %(levelname)s : %(message)s")
        datefmt = kwargs.pop('datefmt', "%m/%d/%Y %H:%M:%S %p")
        self.logfmt = logging.Formatter(fmt=fmt, datefmt=datefmt)

        self.lgr = logging.getLogger()
        self.lgr.setLevel(logging.DEBUG)

        num = kwargs.pop('num', 5)
        self.clearDir(filepath, num)

        t = time.strftime('_%Y_%m_%d_%H_%M_%S', time.localtime())
        filename = filepath+kwargs.pop('filename', 'model')+t+'.log'
        mode = kwargs.pop('mode', 'a')
        fh = self.initHandler(logging.FileHandler, logging.DEBUG,
                              filename=filename, mode=mode, encoding='utf-8')
        self.lgr.addHandler(fh)

        ch = self.initHandler(logging.StreamHandler, logging.INFO)
        self.lgr.addHandler(ch)

    def clearDir(self, filepath, num):

        if not os.path.exists(filepath):
            os.mkdir(filepath)
        elif len(os.listdir(filepath)) > num:
            shutil.rmtree(filepath)
            os.mkdir(filepath)

    def initHandler(self, handler, level, **kwargs):
        h = handler(**kwargs)

        h.setLevel(level)

        h.setFormatter(self.logfmt)

        return h

    def debug(self, msg):
        self.lgr.debug(msg)

    def info(self, msg):
        self.lgr.info(msg)

    def warning(self, msg):
        self.lgr.warning(msg)

    def error(self, msg):
        self.lgr.error(msg)

    def parameters(self, items):
        msg = ''
        dividing = '------------------------'
        index = 0
        for key in items.keys():
            index += 1
            msg += dividing+str(index)+dividing+'\n'
            msg += key+': \n'
            msg += str(items[key])+'\n'

        self.lgr.debug(msg)


def load_yaml(path):
    with open(path, 'r', encoding="utf-8") as f:
        data = f.read()
    return yaml.safe_load(data)


def load_json(path):
    with open(path, 'r', encoding="utf-8") as f:
        return json.load(f)


def load_text(path):
    with open(path, 'r', encoding="utf-8") as f:
        return f.read()


class Config(Munch):
    def __init__(self, path=None,):
        super(Config, self).__init__()
        if path is not None and isinstance(path, str):
            self.add(path)

    def add(self, path, key=None):
        if ".json" in path:
            data = load_json(path)
        else:
            data = load_yaml(path)
        if key is not None:
            data = {key: data}
        self.update(self.fromDict(data))
