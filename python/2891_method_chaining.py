import pandas as pd
from pandas.testing import assert_frame_equal


def findHeavyAnimals(animals: pd.DataFrame) -> pd.DataFrame:
    filtered_df = animals[animals["weight"] > 100]
    sorted_df = filtered_df.sort_values(by="weight", ascending=False)
    return sorted_df[["name"]]


def test_findHeavyAnimals():
    df = pd.DataFrame(
        {
            "name": ["Tatiana", "Khaled", "Alex", "Jonathan", "Stepha", "Tommy"],
            "species": ["Snake", "Giraffe", "Leopard", "Monkey", "Bear", "Panda"],
            "age": [98, 50, 6, 45, 100, 26],
            "weight": [464, 41, 328, 463, 50, 349],
        }
    )
    got = findHeavyAnimals(df)
    expected = pd.DataFrame(
        {
            "name": ["Tatiana", "Jonathan", "Tommy", "Alex"],
        }
    )
    assert_frame_equal(got.reset_index(drop=True), expected.reset_index(drop=True))
