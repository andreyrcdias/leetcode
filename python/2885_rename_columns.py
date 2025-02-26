import pandas as pd
from pandas.testing import assert_frame_equal


def renameColumns(students: pd.DataFrame) -> pd.DataFrame:
    students = students.rename(
        columns={
            "id": "student_id",
            "first": "first_name",
            "last": "last_name",
            "age": "age_in_years",
        },
    )
    return students


def test_renameColumns():
    df = pd.DataFrame(
        {
            "id": [1, 2, 3, 4, 5],
            "first": ["Mason", "Ava", "Taylor", "Georgia", "Thomas"],
            "last": ["King", "Wright", "Hall", "Thompson", "Moore"],
            "age": [6, 7, 16, 18, 10],
        }
    )
    got = renameColumns(df)
    expected = pd.DataFrame(
        {
            "student_id": [1, 2, 3, 4, 5],
            "first_name": ["Mason", "Ava", "Taylor", "Georgia", "Thomas"],
            "last_name": ["King", "Wright", "Hall", "Thompson", "Moore"],
            "age_in_years": [6, 7, 16, 18, 10],
        }
    )
    assert_frame_equal(got, expected)
