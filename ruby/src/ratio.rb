class Integer
    def sign
        if self > 0
            1
        elsif self < 0
            -1
        else
            0
        end
    end
end

class Ratio
    attr_reader :d,:n
    def initialize(n,d)
        if d==0
            @n=1
            @d=0
        else
            g=d.sign*n.abs.gcd(d.abs)
            @n=n/g
            @d=d/g
        end

    end

    def to_str()
        if @d==0
            "NaN"
        else
            if @d==1
                @n.to_s
            else
                @n.to_s+"/"+@d.to_s
            end
        end
    end

    def +(other)
        if @d!=0&&other.d!=0
            Ratio.new(@n * other.d + other.n * @d, @d * other.d)
        else
            Ratio.new(1,0)
        end
    end

    def -(other)
        if @d!=0&&other.d!=0
            Ratio.new(@n * other.d - other.n * @d, @d * other.d)
        else
            Ratio.new(1,0)
        end
    end

    def *(other)
        if @d!=0&&other.d!=0
            Ratio.new(@n*other.n, @d * other.d)
        else
            Ratio.new(1,0)
        end
    end

    def /(other)
        if @d!=0&&other.d!=0
            Ratio.new(@n*other.d, @d * other.n)
        else
            Ratio.new(1,0)
        end
    end
end