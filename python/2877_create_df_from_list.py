from typing import List

import pandas as pd
from pandas.testing import assert_frame_equal


def createDataframe(student_data: List[List[int]]) -> pd.DataFrame:
    df = pd.DataFrame(student_data, columns=["student_id", "age"])
    return df


def test_createDataframe():
    student_data = [
        [1, 15],
        [2, 11],
        [3, 11],
        [4, 20],
    ]
    got = createDataframe(student_data)
    expected = pd.DataFrame(
        {
            "student_id": [1, 2, 3, 4],
            "age": [15, 11, 11, 20],
        }
    )
    assert_frame_equal(got, expected)
