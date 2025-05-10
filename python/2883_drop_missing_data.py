import pandas as pd
from pandas.testing import assert_frame_equal


def dropMissingData(students: pd.DataFrame) -> pd.DataFrame:
    students.dropna(subset=["name"], inplace=True)
    return students.reset_index(drop=True)


def test_dropMissingData():
    df1 = pd.DataFrame(
        {
            "student_id": [32, 217, 779, 849],
            "name": ["Piper", None, "Georgia", "Willow"],
            "age": [5, 19, 20, 14],
        }
    )
    got = dropMissingData(df1)
    expected = pd.DataFrame(
        {
            "student_id": [32, 779, 849],
            "name": ["Piper", "Georgia", "Willow"],
            "age": [5, 20, 14],
        }
    )
    assert_frame_equal(got, expected)
