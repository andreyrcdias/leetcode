from typing import List

import pandas as pd
from pandas.testing import assert_frame_equal


def getDataframeSize(players: pd.DataFrame) -> List[int]:
    return [len(players), len(players.columns)]


def test_getDataframeSize():
    df = pd.DataFrame(
        {
            "player_id": [846, 749, 155, 583, 388, 883, 355, 247, 761, 642],
            "name": [
                "Mason",
                "Riley",
                "Bob",
                "Isabella",
                "Zachary",
                "Ava",
                "Violet",
                "Thomas",
                "Jack",
                "Charlie",
            ],
            "age": [21, 30, 28, 32, 24, 23, 18, 27, 33, 36],
            "position": [
                "Forward",
                "Winger",
                "Striker",
                "Goalkeeper",
                "Midfielder",
                "Defender",
                "Striker",
                "Striker",
                "Midfielder",
                "Center-back",
            ],
            "team": [
                "RealMadrid",
                "Barcelona",
                "MachesterUnited",
                "Liverpool",
                "BayernMunich",
                "Chelsea",
                "Juventus",
                "ParisSaint-Germain",
                "MachesterCity",
                "Arsenal",
            ],
        }
    )
    got = getDataframeSize(df)
    expected = [10, 5]
    assert got == expected
