import json
import os
from itertools import takewhile

import pdfplumber
from pdfplumber.page import Page


def parse_page(page: Page, is_even: bool) -> list[str]:
    height = page.height
    width = page.width

    if is_even:
        cropped_page = page.within_bbox((0, 0, width - 50, height - 40))
    else:
        cropped_page = page.within_bbox((40, 40, width - 112, height - 40))

    col = [name["text"] for name in cropped_page.extract_words(use_text_flow=True)]
    return col


def collect_pairs(arr: list[str]):
    tmp = []

    pairs = []
    for name in arr:
        splits = name.split("-")
        # pp(splits)
        if all(x.isnumeric() for x in splits):
            pairs.append(name)
        else:
            pairs.append(name)
            if len(pairs) != 0:
                tmp.append(pairs.copy())
            pairs.clear()

    return tmp


def parse_list(break_str: str, parsed_names):
    arr = list(takewhile(lambda x: x[0] != break_str, parsed_names))
    cor_len = len(arr)
    arr = list(filter(lambda x: len(x) > 1, arr))
    return arr, parsed_names[cor_len:]


def main() -> None:
    file_path = os.path.dirname(__file__)
    pdf_path = os.path.join(file_path, "assets/FATAL.pdf")
    output_path = os.path.join(file_path, "generated/output.json")

    raw_names = []

    with pdfplumber.open(pdf_path) as pdf:
        # page = pdf.pages[951]
        # cropped_page = page.within_bbox((40, 40, page.width - 112, page.height - 40))
        # cropped_page = page.within_bbox((0, 0, page.width - 50, page.height - 50))
        # cropped_page.to_image().show()
        for n in range(947, 963):
            pages = parse_page(pdf.pages[n], n % 2 == 0)
            raw_names.extend(pages)

    parsed_names = collect_pairs(raw_names)

    human_male, rest = parse_list("Female", parsed_names)
    human_female, rest = parse_list("Last", rest)
    human_last, rest = parse_list("Bugbear", rest)

    # Cut out mid-sentence
    bugbear_male_pre, rest = parse_list("Suffi", rest)
    bugbear_male_suf, rest = parse_list("Prefixes", rest)
    bugbear_female_pre, rest = parse_list("Suffixes", rest)
    bugbear_female_suf, rest = parse_list("Dwarven", rest)

    dwarven_male, rest = parse_list("Female", rest)
    dwarven_female, rest = parse_list("Elven", rest)

    elven_male, rest = parse_list("Female", rest)
    elven_female, rest = parse_list("Kobold", rest)

    kobold_male, rest = parse_list("Female", rest)
    kobold_female, rest = parse_list("Base", rest)

    base_ogre_male_pre, rest = parse_list("Suf", rest)
    base_ogre_male_suf, rest = parse_list("Cliff", rest)

    cliff_ogre_male_pre, rest = parse_list("Suffixes", rest)
    cliff_ogre_male_suf, rest = parse_list("Prefixes", rest)

    grugach_male_pre, rest = parse_list("Suffixes", rest)
    grugach_male_suf, rest = parse_list("Prefixes", rest)

    kinderfresser_male_pre, rest = parse_list("Suffixes", rest)
    kinderfresser_male_suf, rest = parse_list("Prefixes", rest)

    borb_hill_pre, rest = parse_list("Suffixes", rest)
    borb_hill_suf, rest = parse_list("Subterranean", rest)

    sub_male, rest = parse_list("Female", rest)
    sub_female, _rest = parse_list("", rest)

    with open(output_path, "w") as fp:
        json.dump(
            {
                "human": {
                    "male": human_male,
                    "female": human_female,
                    "last": human_last,
                },
                "bugbear": {
                    "male": {
                        "pre": bugbear_male_pre,
                        "suf": bugbear_male_suf,
                    },
                    "female": {"pre": bugbear_female_pre, "suf": bugbear_female_suf},
                },
                "dwarven": {
                    "male": dwarven_male,
                    "female": dwarven_female,
                },
                "elven": {
                    "male": elven_male,
                    "female": elven_female,
                },
                "kobold": {"male": kobold_male, "female": kobold_female},
                "base_ogre": {
                    "pre": base_ogre_male_pre,
                    "suf": base_ogre_male_suf,
                },
                "cliff_ogre": {"pre": cliff_ogre_male_pre, "suf": cliff_ogre_male_suf},
                "grugach_ogre": {
                    "pre": grugach_male_pre,
                    "suf": grugach_male_suf,
                },
                "kinderfresser": {
                    "pre": kinderfresser_male_pre,
                    "suf": kinderfresser_male_suf,
                },
                "borb_hill": {
                    "pre": borb_hill_pre,
                    "suf": borb_hill_suf,
                },
                "sub_troll": {"male": sub_male, "female": sub_female},
            },
            fp,
        )


if __name__ == "__main__":
    main()
