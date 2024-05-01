import os

import psycopg2
from dotenv import load_dotenv


def get_variable_types(connection):
    cursor = connection.cursor()
    cursor.execute(
        """
        SELECT column_name, data_type
        FROM information_schema.columns
        WHERE table_schema = 'public'
    """
    )
    variable_types = cursor.fetchall()
    cursor.close()
    return variable_types


def main():
    load_dotenv()
    try:
        dbname = os.getenv("POSTGRES_DB")
        user = os.getenv("POSTGRES_USER")
        password = os.getenv("POSTGRES_PASSWORD")
        port = os.getenv("PGPORT")

        # Establishing a connection to the PostgreSQL database
        conn = psycopg2.connect(
            dbname=dbname,
            user=user,
            password=password,
            host="localhost",
            port=port,
        )

        # Get variable types
        variable_types = get_variable_types(conn)

        # Print the results
        print("Variable Types:")
        for column_name, data_type in variable_types:
            print(f"{column_name}: {data_type}")

    except psycopg2.Error as e:
        print("Error connecting to the database:", e)

    finally:
        if conn is not None:
            conn.close()


if __name__ == "__main__":
    main()
