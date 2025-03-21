import pandas as pd
from pandas.testing import assert_frame_equal


def changeDatatype(students: pd.DataFrame) -> pd.DataFrame:
    students["grade"] = students["grade"].astype(int)
    return students


def test_changeDatatype():
    df1 = pd.DataFrame(
        {
            "student_id": [1, 2],
            "name": ["Ava", "Kate"],
            "age": [6, 15],
            "grade": [73.0, 87.0],
        }
    )
    got = changeDatatype(df1)
    expected = pd.DataFrame(
        {
            "student_id": [1, 2],
            "name": ["Ava", "Kate"],
            "age": [6, 15],
            "grade": [73, 87],
        }
    )
    assert_frame_equal(got, expected)
