-- OFFSETで2番目のデータ取得
-- サブクエリを使用することでレコードが0件の場合はNullを返却
SELECT (
    SELECT salary
    FROM
        employee
    ORDER BY salary DESC
    LIMIT
        1
        OFFSET 1
) AS SecondHighestSalary
