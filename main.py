import os

import psycopg2
from dotenv import load_dotenv


def get_variable_types(connection, table_name="users"):
    """Get column names and their types for the given table.

    Implementative details:
    - This query handles custom types and return their names."""
    cursor = connection.cursor()
    cursor.execute(
        f"""SELECT 
        a.attname AS column_name,
        format_type(a.atttypid, a.atttypmod) AS data_type
    FROM 
        pg_attribute a
    JOIN 
        pg_class c ON a.attrelid = c.oid
    JOIN 
        pg_namespace n ON n.oid = c.relnamespace
    WHERE 
        c.relname = '{table_name}'
        AND n.nspname = 'public' -- or any other schema name
        AND a.attnum > 0
    ORDER BY 
        a.attnum;"""
    )
    variable_types = cursor.fetchall()
    cursor.close()
    return variable_types


def get_oid_of_Text(connection):
    cursor = connection.cursor()
    cursor.execute(
        f"""SELECT * FROM pg_attribute
        WHERE attrelid = (SELECT oid FROM pg_class WHERE relname = 'mytable');"""
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
    get_oid_of_Text()
