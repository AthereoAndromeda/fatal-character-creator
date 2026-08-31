import json
import os
from pprint import pp

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


def collect_pairs(arr: list[str]) -> list[list[str]]:
    tmp = []
    pairs = []

    for name in arr:
        splits = name.split("-")
        if all(x.isnumeric() for x in splits):
            pairs.append(name)
        else:
            pairs.append(name)
            if len(pairs) != 0:
                tmp.append(pairs.copy())
            pairs.clear()

    return tmp


# Fix list since FATAL is a garbage game with a broken PDF
#
# Fixes:
# - Page 950: 120 overlaps
# - Page 951: Hall range is 301:350
# - Page 955: 754:760 Reduald has gap
# - Page 955: 271 overlaps
# - Page 956: 817,818 missing
# - Page 962: 192 Overlap
def apply_fixes(arr: list[list[str]]) -> list[list[str]]:
    tmp_arr = arr.copy()

    # Remove collision
    inc_idx = tmp_arr.index(["120-123", "Brettingham"])
    tmp_arr[inc_idx] = ["121-123", "Brettingham"]

    # Delete Hall
    tmp_arr.remove(["301-350", "Hall"])

    # Add missing index for Elven male 753
    inc_idx = tmp_arr.index(["754-760", "Reduald"])
    tmp_arr[inc_idx] = ["753-760", "Reduald"]

    # Add missing index for Elven male 817-818
    inc_idx = tmp_arr.index(["819-824", "Sighard"])
    tmp_arr[inc_idx] = ["817-824", "Sighard"]

    # Fix overlap for Elven Male 271-273
    inc_idx = tmp_arr.index(["271-280", "Ceoluulf"])
    tmp_arr[inc_idx] = ["273-280", "Ceoluulf"]

    # Fix overlap for Subterranean troll
    inc_idx = tmp_arr.index(["185-192", "Eileithyia"])
    tmp_arr[inc_idx] = ["185-191", "Eileithyia"]

    return tmp_arr


def get_names(arr: list[list[str]]):
    tmp = []
    last_idx = 0

    for idx in range(1, len(arr)):
        curr_name = arr[idx]
        prev_name = arr[idx - 1]
        last_idx = idx

        # End of names
        if len(curr_name) == 1:
            if len(prev_name) == 2:
                break
            else:
                continue
        else:
            tmp.append(curr_name.copy())

    if not validate_list(tmp):
        raise Exception  # noqa: TRY002

    return tmp, last_idx


def collect_get_names(arr):
    tmp = []
    in_arr = arr

    while True:
        names, idx = get_names(in_arr)
        tmp.append(names)
        in_arr = in_arr[idx:]

        if len(in_arr) == 1:
            break

    return tmp


# Check for missing entries
def validate_list(arr: list[list[str]]) -> bool:
    remaining_count = 1000
    my_list = [x[0] for x in arr]

    start = None
    end = None

    y_end = None
    y_start = None

    len2_count = 0
    for idx in my_list:
        splits = [int(x) for x in idx.split("-")]
        # pp(remaining_count)
        pp(splits)

        if len(splits) == 1:
            remaining_count -= 1
            start = splits[0]
            end = splits[0]
            y_end = splits[0]
            y_start = splits[0]
        elif len(splits) == 2:
            assert splits[1] > splits[0]

            if len2_count % 2 == 0:
                end = splits[1]
                y_start = splits[0]

                # Find Gaps
                if (
                    y_start is not None
                    and y_end is not None
                    and (
                        abs(y_end - y_start) > 1 or y_end > y_start
                    )  # Overlap and Gap check
                ):
                    raise ValueError(f"Gap found: {y_end} - {y_start}")
            else:
                start = splits[0]
                y_end = splits[1]

                if (
                    start is not None
                    and end is not None
                    and (abs(end - start) > 1 or end > start)
                ):
                    raise ValueError(f"Gap found: {end} - {start}")

            # Inclusive start and end.
            if start == end or y_start == y_end:
                raise ValueError(f"Inclusive start and end found: {start}")
            else:
                remaining_count -= (
                    splits[1] - splits[0]
                ) + 1  # Add 1 since end-inclusive

            len2_count += 1
        else:
            raise ValueError("Not a range")

    print(remaining_count)

    # Name is either 1d1000 or 1d100
    return bool(remaining_count == 0 or remaining_count == 900)


def main() -> None:
    file_path = os.path.dirname(__file__)
    pdf_path = os.path.join(file_path, "assets/FATAL.pdf")
    output_path = os.path.join(file_path, "generated/output.json")

    raw_names = []

    with pdfplumber.open(pdf_path) as pdf:
        for n in range(947, 963):
            pages = parse_page(pdf.pages[n], n % 2 == 0)
            raw_names.extend(pages)

    paired_names = collect_pairs(raw_names)
    paired_names = apply_fixes(paired_names)

    names = collect_get_names(paired_names)
    pp(names)

    with open(output_path, "w") as fp:
        json.dump(
            {
                "human": {
                    "male": names[0],
                    "female": names[1],
                    "last": names[2],
                },
                "bugbear": {
                    "male": {
                        "pre": names[3],
                        "suf": names[4],
                    },
                    "female": {"pre": names[5], "suf": names[6]},
                },
                "dwarven": {
                    "male": names[7],
                    "female": names[8],
                },
                "elven": {
                    "male": names[9],
                    "female": names[10],
                },
                "kobold": {"male": names[11], "female": names[12]},
                "base_ogre": {
                    "pre": names[13],
                    "suf": names[14],
                },
                "cliff_ogre": {"pre": names[15], "suf": names[16]},
                "grugach_ogre": {
                    "pre": names[17],
                    "suf": names[18],
                },
                "kinderfresser": {
                    "pre": names[19],
                    "suf": names[20],
                },
                "borb_hill": {
                    "pre": names[21],
                    "suf": names[22],
                },
                "sub_troll": {"male": names[23], "female": names[24]},
            },
            fp,
        )


if __name__ == "__main__":
    main()
