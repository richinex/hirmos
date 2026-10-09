import io
import json
import sys

from fontTools.subset import Options, Subsetter
from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont


def ligatures(font):
    result = {}
    for lookup in font["GSUB"].table.LookupList.Lookup:
        for subtable in lookup.SubTable:
            if subtable.LookupType == 7:
                subtable = subtable.ExtSubTable
            if subtable.LookupType != 4:
                continue
            for first, sequences in subtable.ligatures.items():
                for sequence in sequences:
                    result[(first, *sequence.Component)] = sequence.LigGlyph
    return result


def names(source_path, target_path):
    font = TTFont(source_path)
    characters = {glyph: chr(code) for code, glyph in font.getBestCmap().items()}
    spelled = sorted("".join(characters[glyph] for glyph in spelling) for spelling in ligatures(font))
    json.dump({"names": spelled}, open(target_path, "w"), indent=0)


def subset(names_path, source_path, target_path):
    wanted = json.load(open(names_path))["names"]
    font = TTFont(source_path, lazy=False)
    instantiateVariableFont(font, {"wght": 300, "GRAD": 0, "opsz": 24}, inplace=True)
    instance = io.BytesIO()
    font.flavor = None
    font.save(instance)
    font = TTFont(instance, lazy=False)

    cmap = font.getBestCmap()
    forms = ligatures(font)
    spellings = {name: tuple(cmap[ord(character)] for character in name) for name in wanted}
    missing = [name for name, spelling in spellings.items() if spelling not in forms]
    if missing:
        sys.exit(f"Material Symbols Sharp has no ligature for: {', '.join(missing)}")

    options = Options()
    options.flavor = "woff2"
    options.layout_closure = False
    options.layout_features = ["rlig"]
    options.name_IDs = ["*"]
    options.notdef_outline = True
    subsetter = Subsetter(options)
    subsetter.populate(
        glyphs=sorted({forms[spelling] for spelling in spellings.values()}),
        unicodes=sorted({ord(character) for name in wanted for character in name}),
    )
    subsetter.subset(font)
    font.save(target_path)


commands = {"names": names, "subset": subset}
commands[sys.argv[1]](*sys.argv[2:])
