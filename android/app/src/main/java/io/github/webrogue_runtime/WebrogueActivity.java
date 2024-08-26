package io.github.webrogue_runtime;

import android.graphics.Color;
import android.os.Bundle;
import android.widget.RelativeLayout;
import android.widget.TextView;

import androidx.annotation.Keep;
import org.libsdl.app.SDLActivity;
import java.nio.charset.StandardCharsets;

public class WebrogueActivity extends SDLActivity {
    TextView textView;
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        sharedWebrogueActivity = this;
        super.onCreate(savedInstanceState);
        setWindowStyle(true);
        textView = new TextView(this);
        textView.setText("asbdfb");
        textView.setTextColor(Color.parseColor("#ffd9ff04"));
        RelativeLayout.LayoutParams lp = new  RelativeLayout.LayoutParams(
                RelativeLayout.LayoutParams.WRAP_CONTENT, RelativeLayout.LayoutParams.WRAP_CONTENT
        );
        lp.addRule(RelativeLayout.CENTER_IN_PARENT);
        mLayout.addView(textView, lp);
    }

    private static WebrogueActivity sharedWebrogueActivity;

    @Keep
    public static void printBytes(byte[] bytes) {
        sharedWebrogueActivity.runOnUiThread(() -> {
            sharedWebrogueActivity.textView.setText(new String(bytes, StandardCharsets.UTF_8));
        });
    }
    @Override
    protected String[] getLibraries() {
        return new String[]{ "webrogue" };
    }
}
