import pandas as pd
from pandas.testing import assert_frame_equal


def concatenateTables(df1: pd.DataFrame, df2: pd.DataFrame) -> pd.DataFrame:
    return pd.concat([df1, df2]).reset_index(drop=True)


def test_concatenateTables():
    df1 = pd.DataFrame(
        {
            "student_id": [1, 2, 3, 4],
            "name": ["Mason", "Ava", "Taylor", "Georgia"],
            "age": [8, 6, 15, 17],
        }
    )
    df2 = pd.DataFrame(
        {
            "student_id": [5, 6],
            "name": ["Leo", "Alex"],
            "age": [7, 7],
        }
    )
    got = concatenateTables(df1, df2)
    expected = pd.DataFrame(
        {
            "student_id": [1, 2, 3, 4, 5, 6],
            "name": ["Mason", "Ava", "Taylor", "Georgia", "Leo", "Alex"],
            "age": [8, 6, 15, 17, 7, 7],
        }
    )
    assert_frame_equal(got, expected)
