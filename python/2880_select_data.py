import pandas as pd
from pandas.testing import assert_frame_equal


def selectData(students: pd.DataFrame) -> pd.DataFrame:
    return students.loc[students["student_id"] == 101, ["name", "age"]]


def test_selectData():
    df = pd.DataFrame(
        {
            "student_id": [101, 53, 128, 3],
            "name": ["Ulysses", "William", "Henry", "Henry"],
            "age": [13, 10, 6, 11],
        }
    )
    got = selectData(df)
    expected = pd.DataFrame(
        {
            "name": ["Ulysses"],
            "age": [13],
        }
    )
    assert_frame_equal(got, expected)
